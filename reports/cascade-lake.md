## Benchmark environment

| Environment | Value |
|---|---|
| UTC timestamp | `2026-08-18T11:59:53Z` |
| Commit | `4d979a31fc674d1200e061299d9f2e2e51e522fe` |
| Worktree | dirty |
| Parquet rev (apache/arrow-rs PR 9372) | `f9794b4f4ac9fa896ed507b6d7c6e7556db041a9` |
| CPU | Intel(R) Xeon(R) CPU @ 3.10GHz |
| Architecture | `x86_64` |
| SIMD ISA | `AVX-512F, AVX2, AVX` |
| Logical CPUs | 8 |
| OS and kernel | `Linux 6.17.0-1022-gcp` |
| CPU governor | `unavailable` |
| Rust | `rustc 1.97.0 (2d8144b78 2026-07-07)` |
| LLVM | `22.1.6` |
| Cargo | `cargo 1.97.0 (c980f4866 2026-06-30)` |
| RUSTFLAGS | `-C target-cpu=native` |
| Dataset archive SHA-256 | `1070817918b9e2b2cc7003995927bd04fe7b942045383913d3f40437eda29831` |


## Parquet compression results

| Dataset | Parquet choice | Compression (GB/s) | Decompression (GB/s) | Compressed size (bits/value) |
|---|---|---:|---:|---:|
| arade4 | PLAIN | 12.496 | 12.461 | 64.01 |
| arade4 | PLAIN + ZSTD | 0.290 | 0.941 | 37.39 |
| arade4 | ALP | 1.054 | 8.459 | 24.99 |
| basel_temp_f | PLAIN | 8.386 | 15.904 | 64.01 |
| basel_temp_f | PLAIN + ZSTD | 0.254 | 0.772 | 23.07 |
| basel_temp_f | ALP | 0.600 | 8.204 | 29.23 |
| basel_wind_f | PLAIN | 7.508 | 15.302 | 64.01 |
| basel_wind_f | PLAIN + ZSTD | 0.306 | 0.770 | 18.53 |
| basel_wind_f | ALP | 0.962 | 9.587 | 29.87 |
| bird_migration_f | PLAIN | 13.186 | 21.327 | 64.01 |
| bird_migration_f | PLAIN + ZSTD | 0.301 | 0.815 | 23.49 |
| bird_migration_f | ALP | 1.171 | 9.474 | 20.24 |
| bitcoin_f | PLAIN | 18.883 | 22.191 | 64.07 |
| bitcoin_f | PLAIN + ZSTD | 0.271 | 0.901 | 50.01 |
| bitcoin_f | ALP | 0.796 | 9.182 | 27.18 |
| bitcoin_transactions_f | PLAIN | 5.534 | 14.702 | 64.01 |
| bitcoin_transactions_f | PLAIN + ZSTD | 0.490 | 1.104 | 47.96 |
| bitcoin_transactions_f | ALP | 0.880 | 8.065 | 41.27 |
| city_temperature_f | PLAIN | 11.298 | 11.846 | 64.01 |
| city_temperature_f | PLAIN + ZSTD | 0.293 | 0.634 | 17.67 |
| city_temperature_f | ALP | 1.178 | 9.860 | 10.80 |
| cms1 | PLAIN | 12.415 | 5.822 | 64.01 |
| cms1 | PLAIN + ZSTD | 0.334 | 0.773 | 26.84 |
| cms1 | ALP | 0.633 | 4.299 | 35.19 |
| cms25 | PLAIN | 12.739 | 12.366 | 64.01 |
| cms25 | PLAIN + ZSTD | 0.371 | 1.052 | 58.11 |
| cms25 | ALP | 0.926 | 7.128 | 41.17 |
| cms9 | PLAIN | 12.558 | 12.393 | 64.01 |
| cms9 | PLAIN + ZSTD | 0.330 | 0.682 | 11.71 |
| cms9 | ALP | 1.161 | 9.183 | 12.16 |
| food_prices | PLAIN | 12.535 | 13.132 | 64.01 |
| food_prices | PLAIN + ZSTD | 0.302 | 0.703 | 18.13 |
| food_prices | ALP | 0.555 | 6.336 | 23.20 |
| gov10 | PLAIN | 12.615 | 12.252 | 64.01 |
| gov10 | PLAIN + ZSTD | 0.261 | 0.674 | 29.12 |
| gov10 | ALP | 0.884 | 8.058 | 29.88 |
| gov26 | PLAIN | 13.153 | 12.466 | 64.01 |
| gov26 | PLAIN + ZSTD | 3.175 | 5.443 | 0.20 |
| gov26 | ALP | 0.895 | 15.095 | 1.40 |
| gov30 | PLAIN | 13.043 | 12.378 | 64.01 |
| gov30 | PLAIN + ZSTD | 1.028 | 2.310 | 4.52 |
| gov30 | ALP | 0.576 | 9.830 | 17.88 |
| gov31 | PLAIN | 13.131 | 12.540 | 64.01 |
| gov31 | PLAIN + ZSTD | 1.689 | 3.226 | 1.65 |
| gov31 | ALP | 1.203 | 11.370 | 6.77 |
| gov40 | PLAIN | 13.333 | 12.530 | 64.01 |
| gov40 | PLAIN + ZSTD | 2.877 | 4.773 | 0.43 |
| gov40 | ALP | 1.302 | 14.192 | 2.59 |
| medicare1 | PLAIN | 12.106 | 11.005 | 64.01 |
| medicare1 | PLAIN + ZSTD | 0.284 | 0.801 | 31.68 |
| medicare1 | ALP | 0.610 | 6.173 | 40.46 |
| medicare9 | PLAIN | 12.896 | 12.510 | 64.01 |
| medicare9 | PLAIN + ZSTD | 0.335 | 0.692 | 11.86 |
| medicare9 | ALP | 1.176 | 9.464 | 12.82 |
| neon_air_pressure | PLAIN | 12.828 | 12.329 | 64.01 |
| neon_air_pressure | PLAIN + ZSTD | 0.429 | 1.042 | 11.85 |
| neon_air_pressure | ALP | 1.133 | 8.909 | 16.48 |
| neon_bio_temp_c | PLAIN | 12.920 | 12.313 | 64.01 |
| neon_bio_temp_c | PLAIN + ZSTD | 0.300 | 0.738 | 16.84 |
| neon_bio_temp_c | ALP | 1.186 | 9.793 | 10.81 |
| neon_dew_point_temp | PLAIN | 12.720 | 12.342 | 64.01 |
| neon_dew_point_temp | PLAIN + ZSTD | 0.255 | 0.780 | 23.73 |
| neon_dew_point_temp | ALP | 1.164 | 8.995 | 13.63 |
| neon_pm10_dust | PLAIN | 7.771 | 14.718 | 64.01 |
| neon_pm10_dust | PLAIN + ZSTD | 0.456 | 0.870 | 7.79 |
| neon_pm10_dust | ALP | 0.818 | 11.970 | 8.41 |
| neon_wind_dir | PLAIN | 13.109 | 12.420 | 64.01 |
| neon_wind_dir | PLAIN + ZSTD | 0.260 | 0.726 | 24.41 |
| neon_wind_dir | ALP | 1.147 | 10.827 | 15.94 |
| nyc29 | PLAIN | 13.148 | 11.951 | 64.01 |
| nyc29 | PLAIN + ZSTD | 0.328 | 0.837 | 24.67 |
| nyc29 | ALP | 0.976 | 7.578 | 40.43 |
| poi_lat | PLAIN | 11.782 | 5.157 | 64.01 |
| poi_lat | PLAIN + ZSTD | 0.307 | 0.905 | 57.78 |
| poi_lat | ALP | 0.603 | 2.694 | 88.19 |
| poi_lon | PLAIN | 11.728 | 5.255 | 64.01 |
| poi_lon | PLAIN + ZSTD | 0.381 | 0.966 | 60.44 |
| poi_lon | ALP | 0.699 | 3.148 | 79.12 |
| ssd_hdd_benchmarks_f | PLAIN | 16.327 | 22.835 | 64.02 |
| ssd_hdd_benchmarks_f | PLAIN + ZSTD | 0.334 | 0.695 | 12.98 |
| ssd_hdd_benchmarks_f | ALP | 1.191 | 11.872 | 16.04 |
| stocks_de | PLAIN | 13.103 | 12.488 | 64.01 |
| stocks_de | PLAIN + ZSTD | 0.367 | 0.829 | 10.07 |
| stocks_de | ALP | 0.754 | 9.949 | 11.20 |
| stocks_uk | PLAIN | 13.241 | 12.472 | 64.01 |
| stocks_uk | PLAIN + ZSTD | 0.351 | 0.745 | 11.29 |
| stocks_uk | ALP | 0.523 | 9.660 | 12.75 |
| stocks_usa_c | PLAIN | 13.278 | 12.480 | 64.01 |
| stocks_usa_c | PLAIN + ZSTD | 0.396 | 0.840 | 8.24 |
| stocks_usa_c | ALP | 1.221 | 10.653 | 7.95 |
| **ALL AVG.** | **PLAIN** | **12.326** | **12.996** | **64.01** |
| **ALL AVG.** | **PLAIN + ZSTD** | **0.579** | **1.235** | **22.75** |
| **ALL AVG.** | **ALP** | **0.933** | **9.000** | **24.27** |

GB/s is decimal billions of uncompressed input bytes processed per second; higher is better. Compressed size includes Parquet data-page headers but excludes the file footer. Speed processes every value in pages of up to 131072 values and excludes file I/O. PLAIN + ZSTD includes both stages: PLAIN encoding plus ZSTD compression, and ZSTD decompression plus PLAIN decoding. Short pages are repeated for timing stability and normalized to one page.

## Random access

Time to decode 100 deterministic, uniformly distributed rows from `city_temperature_f` (lower is better). Each lookup starts from the encoded page.

| Parquet choice | 100 random rows (µs) |
|---|---:|
| PLAIN | 4.010 |
| PLAIN + ZSTD | 146419.688 |
| ALP | 16.890 |

PLAIN and ALP reset the page decoder, skip to the selected row, and decode one value. PLAIN + ZSTD additionally decompresses the complete target page for every independent lookup. Encoded pages are already in memory; file I/O and page lookup are excluded.

30 datasets. Arithmetic mean: PLAIN 64.01, PLAIN + ZSTD 22.75, ALP 24.27 bits/value.
Median ALP: 17.88 bits/value. ALP is 0.28x the size of PLAIN and 1.25x the size of PLAIN + ZSTD by geometric mean.
ALP is smaller than PLAIN + ZSTD on 10/30 datasets.
Arithmetic mean compression/decompression speed in GB/s: PLAIN 12.326/12.996, PLAIN + ZSTD 0.579/1.235, ALP 0.933/9.000.
