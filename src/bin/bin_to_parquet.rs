// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0

//! Convert raw little-endian f64 files to one-column Parquet files.
//!
//! For every `.bin` file below the input path, writes PLAIN + ZSTD and ALP
//! Parquet files. The input is streamed in bounded batches.
//!
//! Conversion uses all available cores: input files are converted in
//! parallel, and within each file the two output columns are encoded on
//! dedicated worker threads using the [`ArrowColumnWriter`] pattern described
//! in
//! <https://docs.rs/parquet/latest/parquet/arrow/arrow_writer/struct.ArrowColumnWriter.html>
//!
//! Ported from `parquet/examples/alp_to_parquet.rs` in
//! <https://github.com/apache/arrow-rs/pull/10696>.
//!
//! [`ArrowColumnWriter`]: parquet::arrow::arrow_writer::ArrowColumnWriter

use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{Sender, channel};
use std::thread::JoinHandle;

use arrow_array::{Array, ArrayRef, Float64Array};
use arrow_schema::{DataType, Field, Schema, SchemaRef};
use parquet::arrow::ArrowSchemaConverter;
use parquet::arrow::arrow_writer::{
    ArrowColumnChunk, ArrowLeafColumn, ArrowRowGroupWriterFactory, compute_leaves,
};
use parquet::basic::{Compression, Encoding, ZstdLevel};
use parquet::errors::{ParquetError, Result};
use parquet::file::properties::WriterProperties;
use parquet::file::writer::SerializedFileWriter;

const VALUES_PER_BATCH: usize = 128 * 1024;

fn main() -> Result<()> {
    let mut args = std::env::args_os().skip(1);
    let input = args.next().map(PathBuf::from).unwrap_or_else(|| usage(2));
    let output = args.next().map(PathBuf::from).unwrap_or_else(|| usage(2));
    if args.next().is_some() {
        usage(2);
    }

    if !input.exists() {
        return Err(ParquetError::General(format!(
            "input path does not exist: {}",
            input.display()
        )));
    }
    fs::create_dir_all(&output)?;

    let mut inputs = Vec::new();
    collect_bin_files(&input, &mut inputs)?;
    inputs.sort();
    if inputs.is_empty() {
        return Err(ParquetError::General(format!(
            "no .bin files found below {}",
            input.display()
        )));
    }

    let schema = Arc::new(Schema::new(vec![Field::new(
        "value",
        DataType::Float64,
        false,
    )]));

    let threads = std::thread::available_parallelism()
        .map(|threads| threads.get())
        .unwrap_or(1)
        .min(inputs.len());
    eprintln!("converting {} files with {threads} threads", inputs.len());

    let next_input = AtomicUsize::new(0);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..threads)
            .map(|_| {
                scope.spawn(|| -> Result<()> {
                    loop {
                        let index = next_input.fetch_add(1, Ordering::Relaxed);
                        let Some(input) = inputs.get(index) else {
                            return Ok(());
                        };
                        eprintln!(
                            "[{}/{}] converting {}",
                            index + 1,
                            inputs.len(),
                            input.display()
                        );
                        convert(input, &output, schema.clone())?;
                    }
                })
            })
            .collect();
        workers
            .into_iter()
            .try_for_each(|worker| worker.join().expect("conversion thread panicked"))
    })
}

fn usage(code: i32) -> ! {
    eprintln!(
        "usage: bin_to_parquet <input-dir-or-file> <output-dir>\n\n\
         Input .bin files contain raw little-endian IEEE-754 f64 values."
    );
    std::process::exit(code);
}

fn collect_bin_files(path: &Path, output: &mut Vec<PathBuf>) -> std::io::Result<()> {
    if path.is_file() {
        if path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("bin"))
        {
            output.push(path.to_owned());
        }
        return Ok(());
    }
    for entry in fs::read_dir(path)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_bin_files(&path, output)?;
        } else if path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("bin"))
        {
            output.push(path);
        }
    }
    Ok(())
}

/// One output Parquet file whose single column is encoded on a worker thread.
struct Output {
    writer: SerializedFileWriter<File>,
    factory: ArrowRowGroupWriterFactory,
    row_groups: usize,
}

impl Output {
    fn create(path: PathBuf, schema: &SchemaRef, props: WriterProperties) -> Result<Self> {
        let props = Arc::new(props);
        let parquet_schema = ArrowSchemaConverter::new()
            .with_coerce_types(props.coerce_types())
            .convert(schema)?;
        let writer = SerializedFileWriter::new(
            File::create(&path)?,
            parquet_schema.root_schema_ptr(),
            props,
        )?;
        let factory = ArrowRowGroupWriterFactory::new(&writer, schema.clone());
        Ok(Self {
            writer,
            factory,
            row_groups: 0,
        })
    }
}

/// Encodes one output's column chunk for the current row group on its own
/// thread, fed batches of values over a channel.
struct ColumnWorker {
    send: Sender<ArrowLeafColumn>,
    handle: JoinHandle<Result<ArrowColumnChunk>>,
}

impl ColumnWorker {
    fn start(output: &Output) -> Result<Self> {
        let mut writers = output.factory.create_column_writers(output.row_groups)?;
        let mut writer = writers.pop().ok_or_else(|| {
            ParquetError::General("expected one column writer for the value column".into())
        })?;
        let (send, recv) = channel::<ArrowLeafColumn>();
        let handle = std::thread::spawn(move || {
            for leaf in recv {
                writer.write(&leaf)?;
            }
            writer.close()
        });
        Ok(Self { send, handle })
    }
}

fn start_workers(outputs: &[Output]) -> Result<Vec<ColumnWorker>> {
    outputs.iter().map(ColumnWorker::start).collect()
}

/// Waits for the workers to finish encoding and appends each completed column
/// chunk to a new row group in its output file.
fn close_row_group(workers: Vec<ColumnWorker>, outputs: &mut [Output]) -> Result<()> {
    for (worker, output) in workers.into_iter().zip(outputs.iter_mut()) {
        let ColumnWorker { send, handle } = worker;
        drop(send); // Drop send side to signal termination
        let chunk = handle
            .join()
            .map_err(|_| ParquetError::General("column writer thread panicked".into()))??;
        let mut row_group = output.writer.next_row_group()?;
        chunk.append_to_row_group(&mut row_group)?;
        row_group.close()?;
        output.row_groups += 1;
    }
    Ok(())
}

fn convert(input: &Path, output_dir: &Path, schema: SchemaRef) -> Result<()> {
    let stem = input.file_stem().ok_or_else(|| {
        ParquetError::General(format!("input has no file stem: {}", input.display()))
    })?;
    let stem = stem.to_string_lossy();

    let plain_zstd_props = WriterProperties::builder()
        .set_dictionary_enabled(false)
        .set_encoding(Encoding::PLAIN)
        .set_compression(Compression::ZSTD(ZstdLevel::default()))
        .build();
    let alp_props = WriterProperties::builder()
        .set_dictionary_enabled(false)
        .set_encoding(Encoding::ALP)
        .set_compression(Compression::UNCOMPRESSED)
        .build();

    let mut outputs = vec![
        Output::create(
            output_dir.join(format!("{stem}.plain.zstd.parquet")),
            &schema,
            plain_zstd_props,
        )?,
        Output::create(
            output_dir.join(format!("{stem}.alp.parquet")),
            &schema,
            alp_props,
        )?,
    ];
    // Both files must use the same row-group boundaries; fall back to the
    // classic 1M-row default when only a byte-based limit is configured
    let max_group_rows = outputs
        .iter()
        .filter_map(|output| output.writer.properties().max_row_group_row_count())
        .min()
        .unwrap_or(1024 * 1024);

    // Workers for the in-progress row group, one per output file
    let mut workers: Option<Vec<ColumnWorker>> = None;
    let mut rows_in_group = 0;

    let mut reader = BufReader::new(File::open(input)?);
    let mut bytes = vec![0_u8; VALUES_PER_BATCH * size_of::<f64>()];
    loop {
        let mut bytes_read = 0;
        while bytes_read < bytes.len() {
            let read = reader.read(&mut bytes[bytes_read..])?;
            if read == 0 {
                break;
            }
            bytes_read += read;
        }
        if bytes_read == 0 {
            break;
        }
        if bytes_read % size_of::<f64>() != 0 {
            return Err(ParquetError::General(format!(
                "input size is not a multiple of 8 bytes: {}",
                input.display()
            )));
        }
        let values = bytes[..bytes_read]
            .chunks_exact(size_of::<f64>())
            .map(|chunk| f64::from_le_bytes(chunk.try_into().unwrap()))
            .collect::<Vec<_>>();
        let values: ArrayRef = Arc::new(Float64Array::from(values));

        let mut offset = 0;
        while offset < values.len() {
            if workers.is_none() {
                workers = Some(start_workers(&outputs)?);
                rows_in_group = 0;
            }
            let take = (values.len() - offset).min(max_group_rows - rows_in_group);
            let slice = values.slice(offset, take);

            let mut send_failed = false;
            'send: for worker in workers.as_ref().expect("workers started above") {
                for leaf in compute_leaves(schema.field(0), &slice)? {
                    if worker.send.send(leaf).is_err() {
                        send_failed = true;
                        break 'send;
                    }
                }
            }
            if send_failed {
                // A worker exited early; joining it surfaces the real error
                close_row_group(workers.take().expect("workers started above"), &mut outputs)?;
                return Err(ParquetError::General(
                    "column writer exited before all values were sent".into(),
                ));
            }

            offset += take;
            rows_in_group += take;
            if rows_in_group == max_group_rows {
                close_row_group(workers.take().expect("workers started above"), &mut outputs)?;
            }
        }
    }

    if let Some(workers) = workers.take() {
        close_row_group(workers, &mut outputs)?;
    }
    for output in outputs {
        output.writer.close()?;
    }
    Ok(())
}
