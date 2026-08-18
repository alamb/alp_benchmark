// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

//! Compares the on-disk size of three Parquet choices for floating-point
//! columns: `PLAIN`, `PLAIN + ZSTD`, and `ALP` without a block compressor.
//!
//! Ported from the ALP compression statistics example in
//! <https://github.com/apache/arrow-rs/pull/10696>.
//!
//! # Reproducing the numbers
//!
//! The companion script downloads and verifies the complete CWI ALP corpus,
//! extracts the 30 `f64` datasets used by the paper into a durable, gitignored
//! directory (`./data` by default), and runs this binary:
//!
//! ```shell
//! ./benchmark.sh
//! ```
//!
//! Set `ALP_DATASET_DIR` to use a different dataset directory. Downloads
//! resume if the script is interrupted, and the downloaded 6.7 GiB archive is
//! kept so reruns do not download it again. The archive plus the extracted
//! inputs occupy roughly 22 GiB.
//!
//! These CWI files are raw little-endian IEEE-754 `f64` values. The remaining
//! archive entries are `f32` datasets or a dummy fixture and are outside the
//! raw-binary path of this benchmark. A directory of one-double-per-line CSV
//! files, such as `CWI/ALP/data/samples`, also works. Directories are searched
//! recursively for `.bin`, `.csv`, and `.parquet` files.
//!
//! # Bring your own Parquet data
//!
//! To try the benchmark on your own datasets, pass a Parquet file or a
//! directory containing Parquet files. Every top-level `FLOAT` or `DOUBLE`
//! column becomes its own dataset named `<file>/<column>` and runs the same
//! size, speed, and random-access comparisons as the `.bin` corpus. Other
//! column types are skipped and null values are dropped. `FLOAT` columns are
//! measured natively: four uncompressed bytes per value instead of eight.
//!
//! # What is measured
//!
//! Each input is streamed through a Parquet writer whose output is discarded.
//! The returned Parquet metadata supplies the compressed column
//! chunk size, including data-page headers but excluding the file footer. Using
//! that same boundary for all three choices makes the bits/value figures
//! directly comparable without retaining a potentially multi-gigabyte Parquet
//! file in memory.
//!
//! Speed processes every value in every dataset in 131,072-value pages.
//! Short pages are repeated to stabilize the elapsed-time measurement and
//! normalized back to one page before being added to the dataset total. The
//! reported GB/s uses the uncompressed input size (eight bytes per `f64`
//! value, four per `f32`). The companion script builds with
//! `-C target-cpu=native` unless `RUSTFLAGS` is already set.
//!
//! A focused random-access comparison performs 100 deterministic point lookups
//! on `city_temperature_f` and on every Parquet-sourced column. Each lookup
//! starts from an in-memory encoded page; PLAIN + ZSTD must decompress the
//! complete page, while ALP can skip directly to the vector containing the
//! selected row.

use std::fs::File;
use std::hint::black_box;
use std::io::{BufRead, BufReader, Read, sink};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;

use arrow_array::{Array, ArrayRef, Float32Array, Float64Array, RecordBatch};
use arrow_schema::{DataType, Field, Schema, SchemaRef};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use parquet::arrow::{ArrowWriter, ProjectionMask};
use parquet::basic::Type as PhysicalType;
use parquet::basic::{Compression, Encoding, ZstdLevel};
use parquet::compression::create_codec;
use parquet::data_type::{DataType as ParquetDataType, DoubleType, FloatType};
use parquet::decoding::{Decoder, get_decoder};
use parquet::encoding::{Encoder, get_encoder};
use parquet::errors::{ParquetError, Result};
use parquet::file::properties::WriterProperties;
use parquet::schema::types::{ColumnDescPtr, ColumnDescriptor, ColumnPath, Type};

/// Keeps input and Arrow writer memory bounded for the full CWI corpus.
const INPUT_BATCH_VALUES: usize = 128 * 1024;
/// The page size used by all three speed comparisons.
const SPEED_PAGE_VALUES: usize = 128 * 1024;
const RANDOM_ACCESS_DATASET: &str = "city_temperature_f";
const RANDOM_ACCESS_ROWS: usize = 100;
const RANDOM_ACCESS_TARGET_SECONDS: f64 = 0.05;
const RANDOM_ACCESS_NOTE: &str = "\nPLAIN and ALP reset the page decoder, skip to the selected row, and decode one value. PLAIN + ZSTD additionally decompresses the complete target page for every independent lookup. Encoded pages are already in memory; file I/O and page lookup are excluded.";

/// A floating-point value type measured by this benchmark: `f64` for `.bin`,
/// `.csv`, and Parquet `DOUBLE` columns; `f32` for Parquet `FLOAT` columns.
trait AlpFloat: Copy + Default + 'static {
    type Parquet: ParquetDataType<T = Self>;
    const ARROW_TYPE: DataType;
    const PHYSICAL: PhysicalType;
    fn bits(self) -> u64;
    fn from_f64(value: f64) -> Self;
    fn into_array(values: Vec<Self>) -> ArrayRef;
    /// The non-null values of a projected single-column record batch.
    fn batch_values(array: &dyn Array) -> Result<Vec<Self>>;
}

impl AlpFloat for f64 {
    type Parquet = DoubleType;
    const ARROW_TYPE: DataType = DataType::Float64;
    const PHYSICAL: PhysicalType = PhysicalType::DOUBLE;

    fn bits(self) -> u64 {
        self.to_bits()
    }

    fn from_f64(value: f64) -> Self {
        value
    }

    fn into_array(values: Vec<Self>) -> ArrayRef {
        Arc::new(Float64Array::from(values))
    }

    fn batch_values(array: &dyn Array) -> Result<Vec<Self>> {
        let array = array
            .as_any()
            .downcast_ref::<Float64Array>()
            .ok_or_else(|| ParquetError::General("expected a Float64 column".into()))?;
        Ok(if array.null_count() == 0 {
            array.values().to_vec()
        } else {
            array.iter().flatten().collect()
        })
    }
}

impl AlpFloat for f32 {
    type Parquet = FloatType;
    const ARROW_TYPE: DataType = DataType::Float32;
    const PHYSICAL: PhysicalType = PhysicalType::FLOAT;

    fn bits(self) -> u64 {
        self.to_bits() as u64
    }

    fn from_f64(value: f64) -> Self {
        value as f32
    }

    fn into_array(values: Vec<Self>) -> ArrayRef {
        Arc::new(Float32Array::from(values))
    }

    fn batch_values(array: &dyn Array) -> Result<Vec<Self>> {
        let array = array
            .as_any()
            .downcast_ref::<Float32Array>()
            .ok_or_else(|| ParquetError::General("expected a Float32 column".into()))?;
        Ok(if array.null_count() == 0 {
            array.values().to_vec()
        } else {
            array.iter().flatten().collect()
        })
    }
}

#[derive(Clone, Copy)]
enum Precision {
    F32,
    F64,
}

enum Source {
    /// A `.bin` or `.csv` file of raw `f64` values.
    Raw(PathBuf),
    /// One top-level `FLOAT` or `DOUBLE` column of a Parquet file, identified
    /// by its root schema index.
    Parquet { path: PathBuf, root: usize },
}

/// One benchmarked column of values: a whole `.bin`/`.csv` file, or a single
/// floating-point column of a Parquet file.
struct Dataset {
    name: String,
    source: Source,
    precision: Precision,
}

impl Dataset {
    /// Random access runs on every Parquet column so bring-your-own datasets
    /// get the complete comparison, and on the corpus dataset the report
    /// highlights.
    fn wants_random_access(&self) -> bool {
        matches!(self.source, Source::Parquet { .. }) || self.name == RANDOM_ACCESS_DATASET
    }
}

struct Row {
    name: String,
    num_values: usize,
    plain: u64,
    plain_zstd: u64,
    alp: u64,
}

struct Measurement {
    num_values: usize,
    compressed_bytes: u64,
}

#[derive(Clone, Copy)]
struct Speed {
    compression: f64,
    decompression: f64,
}

struct SpeedRow {
    name: String,
    plain: Speed,
    plain_zstd: Speed,
    alp: Speed,
}

struct RandomAccessRow {
    name: String,
    plain_us: f64,
    plain_zstd_us: f64,
    alp_us: f64,
}

struct RandomAccessPage {
    start: usize,
    num_values: usize,
    plain: bytes::Bytes,
    plain_zstd: bytes::Bytes,
    alp: bytes::Bytes,
}

struct RandomAccessQuery<T> {
    page: usize,
    offset: usize,
    expected: T,
}

#[derive(Default)]
struct TimingTotals {
    bytes: usize,
    compression: f64,
    decompression: f64,
}

impl TimingTotals {
    fn add(&mut self, bytes: usize, compression: f64, decompression: f64) {
        self.bytes += bytes;
        self.compression += compression;
        self.decompression += decompression;
    }

    fn speed(&self) -> Speed {
        let input_gb = self.bytes as f64 / 1_000_000_000.0;
        Speed {
            compression: input_gb / self.compression,
            decompression: input_gb / self.decompression,
        }
    }
}

fn main() -> Result<()> {
    let input = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            eprintln!(
                "usage: benchmark <dataset directory or file>\n\n\
             Accepts .bin files of raw little-endian f64 values, .csv files\n\
             with one double per line, and .parquet files. Every top-level\n\
             FLOAT or DOUBLE column of a Parquet file is benchmarked as its\n\
             own dataset.\n\n\
             Download complete_binaries.zip as described in:\n  \
             https://github.com/cwida/ALP/blob/main/BENCHMARKING.md"
            );
            std::process::exit(2);
        });

    let mut files = Vec::new();
    collect_files(&input, &mut files)
        .unwrap_or_else(|e| panic!("cannot discover datasets in {}: {e}", input.display()));
    files.sort();
    assert!(
        !files.is_empty(),
        "no .bin, .csv, or .parquet files in {}",
        input.display()
    );

    let datasets = expand_datasets(&files);
    assert!(
        !datasets.is_empty(),
        "no benchmarkable datasets in {}",
        input.display()
    );

    let mut rows = Vec::with_capacity(datasets.len());
    let mut speed_rows = Vec::with_capacity(datasets.len());
    let mut random_rows = Vec::new();
    for (idx, dataset) in datasets.iter().enumerate() {
        eprintln!("[{}/{}] measuring {}", idx + 1, datasets.len(), dataset.name);
        let result = match dataset.precision {
            Precision::F64 => run_dataset::<f64>(dataset),
            Precision::F32 => run_dataset::<f32>(dataset),
        };
        match result {
            Ok((row, speed, random)) => {
                rows.push(row);
                speed_rows.push(speed);
                random_rows.extend(random);
            }
            Err(e) => eprintln!("skipping {}: {e}", dataset.name),
        }
    }
    assert!(!rows.is_empty(), "every dataset failed to benchmark");

    print_table(&rows, &speed_rows);
    print_random_access_tables(&random_rows);
    print_summary(&rows, &speed_rows);
    Ok(())
}

/// Runs the size, speed, and (where applicable) random-access comparisons on
/// one dataset of `T` values.
fn run_dataset<T: AlpFloat>(dataset: &Dataset) -> Result<(Row, SpeedRow, Option<RandomAccessRow>)> {
    let schema = Arc::new(Schema::new(vec![Field::new(
        "value",
        T::ARROW_TYPE,
        false,
    )]));
    let row = measure::<T>(dataset, &schema)?;
    let speed = benchmark_dataset::<T>(dataset)?;
    let random = if dataset.wants_random_access() {
        eprintln!("    random access on {}", dataset.name);
        Some(benchmark_random_access::<T>(dataset, row.num_values)?)
    } else {
        None
    };
    Ok((row, speed, random))
}

fn collect_files(path: &Path, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    if path.is_file() {
        if is_dataset(path) {
            out.push(path.to_owned());
        }
        return Ok(());
    }

    for entry in std::fs::read_dir(path)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_files(&path, out)?;
        } else if is_dataset(&path) {
            out.push(path);
        }
    }
    Ok(())
}

fn is_dataset(path: &Path) -> bool {
    has_extension(path, "bin") || has_extension(path, "csv") || has_extension(path, "parquet")
}

fn has_extension(path: &Path, extension: &str) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case(extension))
}

/// Turns each `.bin`/`.csv` file into one dataset and each Parquet file into
/// one dataset per top-level floating-point column. Unreadable Parquet files
/// are skipped with a warning so one bad file does not abort the run.
fn expand_datasets(files: &[PathBuf]) -> Vec<Dataset> {
    let mut datasets = Vec::new();
    for path in files {
        let name = path.file_stem().unwrap().to_string_lossy().into_owned();
        if has_extension(path, "parquet") {
            if let Err(e) = parquet_columns(path, &name, &mut datasets) {
                eprintln!("skipping {}: {e}", path.display());
            }
        } else {
            datasets.push(Dataset {
                name,
                source: Source::Raw(path.clone()),
                precision: Precision::F64,
            });
        }
    }
    datasets
}

fn parquet_columns(path: &Path, stem: &str, out: &mut Vec<Dataset>) -> Result<()> {
    let builder = ParquetRecordBatchReaderBuilder::try_new(File::open(path)?)?;
    let mut found = false;
    for (root, field) in builder.schema().fields().iter().enumerate() {
        let precision = match field.data_type() {
            DataType::Float32 => Precision::F32,
            DataType::Float64 => Precision::F64,
            _ => continue,
        };
        found = true;
        out.push(Dataset {
            name: format!("{stem}/{}", field.name()),
            source: Source::Parquet {
                path: path.to_owned(),
                root,
            },
            precision,
        });
    }
    if !found {
        eprintln!(
            "skipping {}: no top-level FLOAT or DOUBLE columns",
            path.display()
        );
    }
    Ok(())
}

fn measure<T: AlpFloat>(dataset: &Dataset, schema: &SchemaRef) -> Result<Row> {
    let plain = write::<T>(dataset, schema, Encoding::PLAIN, Compression::UNCOMPRESSED)?;
    let plain_zstd = write::<T>(
        dataset,
        schema,
        Encoding::PLAIN,
        Compression::ZSTD(ZstdLevel::default()),
    )?;
    let alp = write::<T>(dataset, schema, Encoding::ALP, Compression::UNCOMPRESSED)?;

    if plain.num_values != plain_zstd.num_values || plain.num_values != alp.num_values {
        return Err(ParquetError::General(format!(
            "{} changed length between encodings",
            dataset.name
        )));
    }

    Ok(Row {
        name: dataset.name.clone(),
        num_values: plain.num_values,
        plain: plain.compressed_bytes,
        plain_zstd: plain_zstd.compressed_bytes,
        alp: alp.compressed_bytes,
    })
}

fn write<T: AlpFloat>(
    dataset: &Dataset,
    schema: &SchemaRef,
    encoding: Encoding,
    compression: Compression,
) -> Result<Measurement> {
    let props = WriterProperties::builder()
        .set_dictionary_enabled(false)
        .set_encoding(encoding)
        .set_compression(compression)
        .build();
    let mut writer = ArrowWriter::try_new(sink(), schema.clone(), Some(props))?;

    let num_values = for_each_batch::<T>(dataset, |values| {
        let batch = RecordBatch::try_new(schema.clone(), vec![T::into_array(values)])?;
        writer.write(&batch)
    })?;
    let metadata = writer.close()?;
    if num_values == 0 {
        return Err(ParquetError::General(format!(
            "{} contains no values",
            dataset.name
        )));
    }
    let compressed_bytes = metadata
        .row_groups()
        .iter()
        .map(|row_group| row_group.column(0).compressed_size())
        .try_fold(0u64, |total, bytes| {
            let bytes = u64::try_from(bytes).map_err(|_| {
                ParquetError::General(format!("negative column size for {}", dataset.name))
            })?;
            total.checked_add(bytes).ok_or_else(|| {
                ParquetError::General(format!("column size overflow for {}", dataset.name))
            })
        })?;

    Ok(Measurement {
        num_values,
        compressed_bytes,
    })
}

fn for_each_batch<T: AlpFloat>(
    dataset: &Dataset,
    consume: impl FnMut(Vec<T>) -> Result<()>,
) -> Result<usize> {
    match &dataset.source {
        Source::Raw(path) => match path.extension().and_then(|ext| ext.to_str()) {
            Some(ext) if ext.eq_ignore_ascii_case("bin") => read_binary(path, consume),
            Some(ext) if ext.eq_ignore_ascii_case("csv") => read_csv(path, consume),
            _ => Err(ParquetError::General(format!(
                "unsupported dataset file {}",
                path.display()
            ))),
        },
        Source::Parquet { path, root } => read_parquet_column(path, *root, consume),
    }
}

fn read_binary<T: AlpFloat>(
    path: &Path,
    mut consume: impl FnMut(Vec<T>) -> Result<()>,
) -> Result<usize> {
    let mut reader = BufReader::new(File::open(path)?);
    let mut bytes = vec![0u8; INPUT_BATCH_VALUES * std::mem::size_of::<f64>()];
    let mut total = 0usize;

    loop {
        let mut filled = 0;
        while filled < bytes.len() {
            let read = reader.read(&mut bytes[filled..])?;
            if read == 0 {
                break;
            }
            filled += read;
        }
        if filled == 0 {
            break;
        }
        if filled % std::mem::size_of::<f64>() != 0 {
            return Err(ParquetError::General(format!(
                "{} has {} trailing bytes; expected raw little-endian f64 values",
                path.display(),
                filled % std::mem::size_of::<f64>()
            )));
        }

        let values: Vec<T> = bytes[..filled]
            .chunks_exact(std::mem::size_of::<f64>())
            .map(|chunk| T::from_f64(f64::from_le_bytes(chunk.try_into().unwrap())))
            .collect();
        total += values.len();
        consume(values)?;

        if filled < bytes.len() {
            break;
        }
    }
    Ok(total)
}

fn read_csv<T: AlpFloat>(
    path: &Path,
    mut consume: impl FnMut(Vec<T>) -> Result<()>,
) -> Result<usize> {
    let reader = BufReader::new(File::open(path)?);
    let mut values = Vec::with_capacity(INPUT_BATCH_VALUES);
    let mut total = 0usize;

    for (idx, line) in reader.lines().enumerate() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let value = line.parse::<f64>().map_err(|e| {
            ParquetError::General(format!(
                "{}:{}: cannot parse {line:?} as f64: {e}",
                path.display(),
                idx + 1
            ))
        })?;
        values.push(T::from_f64(value));
        if values.len() == INPUT_BATCH_VALUES {
            total += values.len();
            consume(std::mem::take(&mut values))?;
            values = Vec::with_capacity(INPUT_BATCH_VALUES);
        }
    }

    if !values.is_empty() {
        total += values.len();
        consume(values)?;
    }
    Ok(total)
}

fn read_parquet_column<T: AlpFloat>(
    path: &Path,
    root: usize,
    mut consume: impl FnMut(Vec<T>) -> Result<()>,
) -> Result<usize> {
    let builder = ParquetRecordBatchReaderBuilder::try_new(File::open(path)?)?
        .with_batch_size(INPUT_BATCH_VALUES);
    let mask = ProjectionMask::roots(builder.parquet_schema(), [root]);
    let mut total = 0usize;
    for batch in builder.with_projection(mask).build()? {
        let values = T::batch_values(batch?.column(0).as_ref())?;
        if values.is_empty() {
            continue;
        }
        total += values.len();
        consume(values)?;
    }
    Ok(total)
}

fn bits_per_value(bytes: u64, num_values: usize) -> f64 {
    bytes as f64 * 8.0 / num_values as f64
}

fn print_table(rows: &[Row], speed_rows: &[SpeedRow]) {
    assert_eq!(rows.len(), speed_rows.len());
    println!("\n## Parquet compression results\n");
    println!(
        "| Dataset | Parquet choice | Compression (GB/s) | Decompression (GB/s) | Compressed size (bits/value) |"
    );
    println!("|---|---|---:|---:|---:|");
    for (row, speed) in rows.iter().zip(speed_rows) {
        assert_eq!(row.name, speed.name);
        print_result_row(
            &row.name,
            "PLAIN",
            speed.plain,
            bits_per_value(row.plain, row.num_values),
        );
        print_result_row(
            &row.name,
            "PLAIN + ZSTD",
            speed.plain_zstd,
            bits_per_value(row.plain_zstd, row.num_values),
        );
        print_result_row(
            &row.name,
            "ALP",
            speed.alp,
            bits_per_value(row.alp, row.num_values),
        );
    }

    let (plain_bits, plain_zstd_bits, alp_bits) = arithmetic_means(rows);
    let (plain_speed, plain_zstd_speed, alp_speed) = speed_arithmetic_means(speed_rows);
    print_average_row("PLAIN", plain_speed, plain_bits);
    print_average_row("PLAIN + ZSTD", plain_zstd_speed, plain_zstd_bits);
    print_average_row("ALP", alp_speed, alp_bits);

    println!(
        "\nGB/s is decimal billions of uncompressed input bytes processed per second; higher is better. Compressed size includes Parquet data-page headers but excludes the file footer. Speed processes every value in pages of up to {SPEED_PAGE_VALUES} values and excludes file I/O. PLAIN + ZSTD includes both stages: PLAIN encoding plus ZSTD compression, and ZSTD decompression plus PLAIN decoding. Short pages are repeated for timing stability and normalized to one page."
    );
}

fn print_result_row(dataset: &str, choice: &str, speed: Speed, bits: f64) {
    println!(
        "| {dataset} | {choice} | {:.3} | {:.3} | {bits:.2} |",
        speed.compression, speed.decompression
    );
}

fn print_average_row(choice: &str, speed: Speed, bits: f64) {
    println!(
        "| **ALL AVG.** | **{choice}** | **{:.3}** | **{:.3}** | **{bits:.2}** |",
        speed.compression, speed.decompression
    );
}

fn print_random_access_tables(rows: &[RandomAccessRow]) {
    match rows {
        [] => {}
        [row] => print_single_random_access_table(row),
        rows => print_multi_random_access_table(rows),
    }
}

fn print_single_random_access_table(row: &RandomAccessRow) {
    println!("\n## Random access\n");
    println!(
        "Time to decode {RANDOM_ACCESS_ROWS} deterministic, uniformly distributed rows from `{}` (lower is better). Each lookup starts from the encoded page.\n",
        row.name
    );
    println!("| Parquet choice | {RANDOM_ACCESS_ROWS} random rows (µs) |");
    println!("|---|---:|");
    println!("| PLAIN | {:.3} |", row.plain_us);
    println!("| PLAIN + ZSTD | {:.3} |", row.plain_zstd_us);
    println!("| ALP | {:.3} |", row.alp_us);
    println!("{RANDOM_ACCESS_NOTE}");
}

fn print_multi_random_access_table(rows: &[RandomAccessRow]) {
    println!("\n## Random access\n");
    println!(
        "Time to decode {RANDOM_ACCESS_ROWS} deterministic, uniformly distributed rows from each dataset (lower is better). Each lookup starts from the encoded page.\n"
    );
    println!("| Dataset | PLAIN (µs) | PLAIN + ZSTD (µs) | ALP (µs) |");
    println!("|---|---:|---:|---:|");
    for row in rows {
        println!(
            "| {} | {:.3} | {:.3} | {:.3} |",
            row.name, row.plain_us, row.plain_zstd_us, row.alp_us
        );
    }
    println!("{RANDOM_ACCESS_NOTE}");
}

fn print_summary(rows: &[Row], speed_rows: &[SpeedRow]) {
    let (plain_mean, plain_zstd_mean, alp_mean) = arithmetic_means(rows);
    let (plain_speed, plain_zstd_speed, alp_speed) = speed_arithmetic_means(speed_rows);
    let mut alp_bits: Vec<f64> = rows
        .iter()
        .map(|row| bits_per_value(row.alp, row.num_values))
        .collect();
    alp_bits.sort_by(f64::total_cmp);
    let median_alp = alp_bits[alp_bits.len() / 2];

    let alp_vs_plain_geomean = (rows
        .iter()
        .map(|row| (row.alp as f64 / row.plain as f64).ln())
        .sum::<f64>()
        / rows.len() as f64)
        .exp();
    let alp_vs_zstd_geomean = (rows
        .iter()
        .map(|row| (row.alp as f64 / row.plain_zstd as f64).ln())
        .sum::<f64>()
        / rows.len() as f64)
        .exp();
    let beats_zstd = rows.iter().filter(|row| row.alp < row.plain_zstd).count();

    println!(
        "\n{} datasets. Arithmetic mean: PLAIN {plain_mean:.2}, PLAIN + ZSTD {plain_zstd_mean:.2}, ALP {alp_mean:.2} bits/value.",
        rows.len(),
    );
    println!(
        "Median ALP: {median_alp:.2} bits/value. ALP is {:.2}x the size of PLAIN and {:.2}x the size of PLAIN + ZSTD by geometric mean.",
        alp_vs_plain_geomean, alp_vs_zstd_geomean,
    );
    println!(
        "ALP is smaller than PLAIN + ZSTD on {beats_zstd}/{} datasets.",
        rows.len()
    );
    println!(
        "Arithmetic mean compression/decompression speed in GB/s: PLAIN {:.3}/{:.3}, PLAIN + ZSTD {:.3}/{:.3}, ALP {:.3}/{:.3}.",
        plain_speed.compression,
        plain_speed.decompression,
        plain_zstd_speed.compression,
        plain_zstd_speed.decompression,
        alp_speed.compression,
        alp_speed.decompression,
    );
}

fn arithmetic_means(rows: &[Row]) -> (f64, f64, f64) {
    let count = rows.len() as f64;
    let plain = rows
        .iter()
        .map(|row| bits_per_value(row.plain, row.num_values))
        .sum::<f64>()
        / count;
    let plain_zstd = rows
        .iter()
        .map(|row| bits_per_value(row.plain_zstd, row.num_values))
        .sum::<f64>()
        / count;
    let alp = rows
        .iter()
        .map(|row| bits_per_value(row.alp, row.num_values))
        .sum::<f64>()
        / count;
    (plain, plain_zstd, alp)
}

fn speed_arithmetic_means(rows: &[SpeedRow]) -> (Speed, Speed, Speed) {
    let average = |select: fn(&SpeedRow) -> Speed| Speed {
        compression: rows.iter().map(|row| select(row).compression).sum::<f64>()
            / rows.len() as f64,
        decompression: rows
            .iter()
            .map(|row| select(row).decompression)
            .sum::<f64>()
            / rows.len() as f64,
    };
    (
        average(|row| row.plain),
        average(|row| row.plain_zstd),
        average(|row| row.alp),
    )
}

fn benchmark_random_access<T: AlpFloat>(
    dataset: &Dataset,
    num_values: usize,
) -> Result<RandomAccessRow> {
    let descriptor = column_descriptor::<T>()?;
    let indices = random_row_indices(num_values);
    let mut expected: Vec<Option<T>> = vec![None; indices.len()];
    let mut pages = Vec::new();
    let mut page_start = 0usize;

    let mut plain_encoder = get_encoder::<T::Parquet>(Encoding::PLAIN, &descriptor)?;
    let mut alp_encoder = get_encoder::<T::Parquet>(Encoding::ALP, &descriptor)?;
    let mut codec = create_codec(Compression::ZSTD(ZstdLevel::default()), &Default::default())?
        .expect("ZSTD is a compressed codec");
    let mut alp_preset_ready = false;

    let read_values = for_each_batch::<T>(dataset, |values| {
        if !alp_preset_ready {
            alp_encoder.put(&values)?;
            black_box(alp_encoder.flush_buffer()?);
            alp_preset_ready = true;
        }

        let page_end = page_start + values.len();
        let selected = indices
            .iter()
            .any(|&index| (page_start..page_end).contains(&index));
        if selected {
            plain_encoder.put(&values)?;
            let plain = plain_encoder.flush_buffer()?;
            alp_encoder.put(&values)?;
            let alp = alp_encoder.flush_buffer()?;

            let mut plain_zstd = Vec::new();
            codec.compress(plain.as_ref(), &mut plain_zstd)?;
            pages.push(RandomAccessPage {
                start: page_start,
                num_values: values.len(),
                plain,
                plain_zstd: plain_zstd.into(),
                alp,
            });

            for (query, &index) in indices.iter().enumerate() {
                if (page_start..page_end).contains(&index) {
                    expected[query] = Some(values[index - page_start]);
                }
            }
        }
        page_start = page_end;
        Ok(())
    })?;

    if read_values != num_values {
        return Err(ParquetError::General(format!(
            "{} changed length between size and random-access passes",
            dataset.name
        )));
    }

    let queries = indices
        .into_iter()
        .zip(expected)
        .map(|(index, expected)| {
            let page = pages
                .iter()
                .position(|page| (page.start..page.start + page.num_values).contains(&index))
                .expect("every random row belongs to a retained page");
            RandomAccessQuery {
                page,
                offset: index - pages[page].start,
                expected: expected.expect("every random row was read"),
            }
        })
        .collect::<Vec<_>>();

    let mut plain_decoder: Box<dyn Decoder<T::Parquet>> =
        get_decoder(descriptor.clone(), Encoding::PLAIN)?;
    decode_random_rows(&mut plain_decoder, &pages, &queries, Encoding::PLAIN, true)?;
    let plain_us = measure_operation(|| {
        decode_random_rows(&mut plain_decoder, &pages, &queries, Encoding::PLAIN, false)
    })?;

    let mut alp_decoder: Box<dyn Decoder<T::Parquet>> = get_decoder(descriptor, Encoding::ALP)?;
    decode_random_rows(&mut alp_decoder, &pages, &queries, Encoding::ALP, true)?;
    let alp_us = measure_operation(|| {
        decode_random_rows(&mut alp_decoder, &pages, &queries, Encoding::ALP, false)
    })?;

    let mut decompressed = Vec::new();
    decompress_random_pages(&mut codec, &pages, &queries, &mut decompressed, true)?;
    let zstd_us = measure_operation(|| {
        decompress_random_pages(&mut codec, &pages, &queries, &mut decompressed, false)
    })?;

    Ok(RandomAccessRow {
        name: dataset.name.clone(),
        plain_us,
        plain_zstd_us: zstd_us + plain_us,
        alp_us,
    })
}

fn random_row_indices(num_values: usize) -> Vec<usize> {
    let mut state = 0x4d59_5df4_d0f3_3173u64;
    (0..RANDOM_ACCESS_ROWS)
        .map(|_| {
            state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
            let mut value = state;
            value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
            value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
            value ^= value >> 31;
            ((value as u128 * num_values as u128) >> 64) as usize
        })
        .collect()
}

fn decode_random_rows<T: AlpFloat>(
    decoder: &mut Box<dyn Decoder<T::Parquet>>,
    pages: &[RandomAccessPage],
    queries: &[RandomAccessQuery<T>],
    encoding: Encoding,
    validate: bool,
) -> Result<()> {
    let mut decoded = [T::default()];
    for query in queries {
        let page = &pages[query.page];
        let data = match encoding {
            Encoding::PLAIN => &page.plain,
            Encoding::ALP => &page.alp,
            _ => unreachable!(),
        };
        decoder.set_data(data.clone(), page.num_values)?;
        let skipped = decoder.skip(query.offset)?;
        let read = decoder.get(&mut decoded)?;
        if skipped != query.offset || read != 1 {
            return Err(ParquetError::General(format!(
                "{encoding} random lookup skipped {skipped} and read {read} values"
            )));
        }
        if validate && decoded[0].bits() != query.expected.bits() {
            return Err(ParquetError::General(format!(
                "{encoding} random lookup did not reproduce row {}",
                page.start + query.offset
            )));
        }
        black_box(decoded[0]);
    }
    Ok(())
}

fn decompress_random_pages<T: AlpFloat>(
    codec: &mut Box<dyn parquet::compression::Codec>,
    pages: &[RandomAccessPage],
    queries: &[RandomAccessQuery<T>],
    decompressed: &mut Vec<u8>,
    validate: bool,
) -> Result<()> {
    for query in queries {
        let page = &pages[query.page];
        decompressed.clear();
        let read = codec.decompress(
            page.plain_zstd.as_ref(),
            decompressed,
            Some(page.plain.len()),
        )?;
        if read != page.plain.len() {
            return Err(ParquetError::General(format!(
                "ZSTD random lookup decompressed {read} of {} bytes",
                page.plain.len()
            )));
        }
        if validate && decompressed != page.plain.as_ref() {
            return Err(ParquetError::General(
                "ZSTD random lookup did not reproduce the PLAIN page".into(),
            ));
        }
        black_box(decompressed[query.offset * std::mem::size_of::<T>()]);
    }
    Ok(())
}

fn measure_operation(mut operation: impl FnMut() -> Result<()>) -> Result<f64> {
    operation()?;
    let start = Instant::now();
    operation()?;
    let estimate = start.elapsed().as_secs_f64();
    let repetitions = if estimate == 0.0 {
        10_000
    } else {
        (RANDOM_ACCESS_TARGET_SECONDS / estimate)
            .ceil()
            .clamp(3.0, 10_000.0) as usize
    };

    let start = Instant::now();
    for _ in 0..repetitions {
        operation()?;
    }
    Ok(start.elapsed().as_secs_f64() * 1_000_000.0 / repetitions as f64)
}

fn column_descriptor<T: AlpFloat>() -> Result<ColumnDescPtr> {
    let primitive = Type::primitive_type_builder("value", T::PHYSICAL).build()?;
    Ok(Arc::new(ColumnDescriptor::new(
        Arc::new(primitive),
        0,
        0,
        ColumnPath::new(vec!["value".into()]),
    )))
}

fn benchmark_dataset<T: AlpFloat>(dataset: &Dataset) -> Result<SpeedRow> {
    let descriptor = column_descriptor::<T>()?;
    let mut plain_encoder = get_encoder::<T::Parquet>(Encoding::PLAIN, &descriptor)?;
    let mut plain_decoder: Box<dyn Decoder<T::Parquet>> =
        get_decoder(descriptor.clone(), Encoding::PLAIN)?;
    let mut alp_encoder = get_encoder::<T::Parquet>(Encoding::ALP, &descriptor)?;
    let mut alp_decoder: Box<dyn Decoder<T::Parquet>> =
        get_decoder(descriptor.clone(), Encoding::ALP)?;
    let mut codec = create_codec(Compression::ZSTD(ZstdLevel::default()), &Default::default())?
        .expect("ZSTD is a compressed codec");
    let mut plain_totals = TimingTotals::default();
    let mut zstd_totals = TimingTotals::default();
    let mut alp_totals = TimingTotals::default();
    let mut alp_preset_ready = false;

    let num_values = for_each_batch::<T>(dataset, |values| {
        if !alp_preset_ready {
            // Build the row-group preset outside the timed region, matching the
            // paper's exclusion of first-level sampling from compression speed.
            alp_encoder.put(&values)?;
            black_box(alp_encoder.flush_buffer()?);
            alp_preset_ready = true;
        }

        let input_bytes = values.len() * std::mem::size_of::<T>();
        let repetitions = SPEED_PAGE_VALUES.div_ceil(values.len());
        let (plain_page, compression, decompression) = benchmark_encoded_page(
            &values,
            &mut plain_encoder,
            &mut plain_decoder,
            Encoding::PLAIN,
            repetitions,
        )?;
        plain_totals.add(input_bytes, compression, decompression);

        let (zstd_compression, zstd_decompression) =
            benchmark_zstd_page(&plain_page, &mut codec, repetitions)?;
        zstd_totals.add(
            input_bytes,
            compression + zstd_compression,
            zstd_decompression + decompression,
        );

        let (_, compression, decompression) = benchmark_encoded_page(
            &values,
            &mut alp_encoder,
            &mut alp_decoder,
            Encoding::ALP,
            repetitions,
        )?;
        alp_totals.add(input_bytes, compression, decompression);
        Ok(())
    })?;

    if num_values == 0 {
        return Err(ParquetError::General(format!(
            "{} contains no values",
            dataset.name
        )));
    }

    Ok(SpeedRow {
        name: dataset.name.clone(),
        plain: plain_totals.speed(),
        plain_zstd: zstd_totals.speed(),
        alp: alp_totals.speed(),
    })
}

fn benchmark_encoded_page<T: AlpFloat>(
    values: &[T],
    encoder: &mut Box<dyn Encoder<T::Parquet>>,
    decoder: &mut Box<dyn Decoder<T::Parquet>>,
    encoding: Encoding,
    repetitions: usize,
) -> Result<(bytes::Bytes, f64, f64)> {
    let start = Instant::now();
    let mut page = bytes::Bytes::new();
    for _ in 0..repetitions {
        encoder.put(black_box(values))?;
        page = encoder.flush_buffer()?;
        black_box(page.len());
    }
    let compression = elapsed_seconds(start, repetitions)?;

    let mut decoded = vec![T::default(); values.len()];
    let start = Instant::now();
    for _ in 0..repetitions {
        decoder.set_data(page.clone(), values.len())?;
        let read = decoder.get(&mut decoded)?;
        if read != values.len() {
            return Err(ParquetError::General(format!(
                "{encoding} decoded {read} of {} values",
                values.len()
            )));
        }
        black_box(decoded[0]);
    }
    let decompression = elapsed_seconds(start, repetitions)?;
    assert_bits_eq(values, &decoded, encoding)?;

    Ok((page, compression, decompression))
}

fn benchmark_zstd_page(
    plain: &bytes::Bytes,
    codec: &mut Box<dyn parquet::compression::Codec>,
    repetitions: usize,
) -> Result<(f64, f64)> {
    let mut compressed = Vec::new();
    let start = Instant::now();
    for _ in 0..repetitions {
        compressed.clear();
        codec.compress(black_box(plain.as_ref()), &mut compressed)?;
        black_box(compressed.len());
    }
    let compression = elapsed_seconds(start, repetitions)?;

    let mut decompressed = Vec::with_capacity(plain.len());
    let start = Instant::now();
    for _ in 0..repetitions {
        decompressed.clear();
        let read = codec.decompress(
            black_box(compressed.as_slice()),
            &mut decompressed,
            Some(plain.len()),
        )?;
        if read != plain.len() {
            return Err(ParquetError::General(format!(
                "ZSTD decompressed {read} of {} bytes",
                plain.len()
            )));
        }
        black_box(decompressed[0]);
    }
    let decompression = elapsed_seconds(start, repetitions)?;
    if decompressed != plain.as_ref() {
        return Err(ParquetError::General(
            "ZSTD did not reproduce the PLAIN page".into(),
        ));
    }

    Ok((compression, decompression))
}

fn elapsed_seconds(start: Instant, repetitions: usize) -> Result<f64> {
    let seconds = start.elapsed().as_secs_f64() / repetitions as f64;
    if seconds == 0.0 {
        return Err(ParquetError::General(
            "elapsed-time clock did not advance".into(),
        ));
    }
    Ok(seconds)
}

fn assert_bits_eq<T: AlpFloat>(expected: &[T], actual: &[T], encoding: Encoding) -> Result<()> {
    if expected
        .iter()
        .zip(actual)
        .all(|(left, right)| left.bits() == right.bits())
    {
        return Ok(());
    }
    Err(ParquetError::General(format!(
        "{encoding} speed fixture failed to round-trip"
    )))
}
