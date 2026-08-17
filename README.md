# alp_benchmark
Benchmark to support [ALP: Adaptive Light-weight Floating-point Encoding in Apache Parquet](https://github.com/apache/parquet-site/pull/195) (TODO update link
to real site when published) by [Kosta Tarasov](https://github.com/sdf-jkl), [Andrew Lamb](https://github.com/alamb), [Prateek Gaur](https://github.com/prtkgaur)


This repository contains a benchmark for evaluating the performance of the
Adaptive Lossless Floating Point (ALP) encoding in Apache Parquet (TODO GET RELEVANT LINKS). 

It supports the Parquet Blog Post: 

- Related to https://github.com/apache/parquet-site/issues/175
- Related to https://github.com/apache/parquet-site/pull/195

The benchmark compares different encoding and compression strategies for columns
of 32-bit and 64-bit floating point values.

(TODO preview diagrams of the benchmark results)

# Prerequisites

# Benchmark
To run:
```shell
./benchmark.sh
```

This will print a textual report. Example reports:
* [kosta](reports/kosta.md)
* [alamb](reports/alamb.md)

## Diagrams
There is a python script that post-processes the benchmark results and generates diagrams.

```shell
# Create diagrams from kosta.md in diagrams/kosta
./diagrams.sh reports/kosta.md
```






# Related work

* Andrew Lamb's script updates: https://github.com/apache/arrow-rs/pull/10696
# Which issue does this PR close?



# Rationale for this change

@sdf-jkl created a scirpt to measure ALP performance (see here) https://github.com/apache/parquet-site/pull/195#issuecomment-5223205213




# Are there any user-facing changes?


# Datasets Used

TODO (get from blog)