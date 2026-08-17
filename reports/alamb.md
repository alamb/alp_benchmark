## Benchmark environment

| Environment | Value |
|---|---|
| UTC timestamp | `2026-08-15T09:38:17Z` |
| Commit | `0ffde54952cc6fc6ac0fd25a739b4a43022a770b` |
| Worktree | dirty |
| CPU | Intel(R) Xeon(R) CPU @ 3.10GHz |
| Architecture | `x86_64` |
| SIMD ISA | `AVX-512F, AVX2, AVX` |
| Logical CPUs | 8 |
| OS and kernel | `Linux 6.17.0-1022-gcp` |
| CPU governor | `unavailable` |
| Rust | `rustc 1.96.1 (31fca3adb 2026-06-26)` |
| LLVM | `22.1.2` |
| Cargo | `cargo 1.96.1 (356927216 2026-06-26)` |
| RUSTFLAGS | `-C target-cpu=native` |
| Dataset archive SHA-256 | `1070817918b9e2b2cc7003995927bd04fe7b942045383913d3f40437eda29831` |


## Parquet compression results

| Dataset | Parquet choice | Compression (GB/s) | Decompression (GB/s) | Compressed size (bits/value) |
|---|---|---:|---:|---:|
| arade4 | PLAIN | 13.428 | 4.618 | 64.01 |
| arade4 | PLAIN + ZSTD | 0.304 | 0.768 | 37.39 |
| arade4 | ALP | 1.146 | 4.222 | 24.99 |
| basel_temp_f | PLAIN | 14.190 | 15.634 | 64.01 |
| basel_temp_f | PLAIN + ZSTD | 0.261 | 0.703 | 23.07 |
| basel_temp_f | ALP | 0.660 | 9.281 | 29.23 |
| basel_wind_f | PLAIN | 15.096 | 15.333 | 64.01 |
| basel_wind_f | PLAIN + ZSTD | 0.322 | 0.807 | 18.53 |
| basel_wind_f | ALP | 1.066 | 10.160 | 29.87 |
| bird_migration_f | PLAIN | 15.472 | 22.338 | 64.01 |
| bird_migration_f | PLAIN + ZSTD | 0.295 | 0.816 | 23.49 |
| bird_migration_f | ALP | 1.158 | 5.688 | 20.24 |
| bitcoin_f | PLAIN | 19.500 | 21.789 | 64.07 |
| bitcoin_f | PLAIN + ZSTD | 0.265 | 0.909 | 50.01 |
| bitcoin_f | ALP | 0.825 | 8.965 | 27.18 |
| bitcoin_transactions_f | PLAIN | 6.660 | 14.798 | 64.01 |
| bitcoin_transactions_f | PLAIN + ZSTD | 0.520 | 1.147 | 47.96 |
| bitcoin_transactions_f | ALP | 0.958 | 7.396 | 41.27 |
| city_temperature_f | PLAIN | 13.562 | 4.369 | 64.01 |
| city_temperature_f | PLAIN + ZSTD | 0.302 | 0.561 | 17.67 |
| city_temperature_f | ALP | 1.233 | 4.532 | 10.80 |
| cms1 | PLAIN | 13.507 | 6.339 | 64.01 |
| cms1 | PLAIN + ZSTD | 0.353 | 0.804 | 26.84 |
| cms1 | ALP | 0.683 | 4.894 | 35.19 |
| cms25 | PLAIN | 13.829 | 12.360 | 64.01 |
| cms25 | PLAIN + ZSTD | 0.407 | 1.100 | 58.11 |
| cms25 | ALP | 1.020 | 8.744 | 41.17 |
| cms9 | PLAIN | 13.907 | 12.828 | 64.01 |
| cms9 | PLAIN + ZSTD | 0.354 | 0.702 | 11.71 |
| cms9 | ALP | 1.224 | 11.271 | 12.16 |
| food_prices | PLAIN | 13.603 | 8.478 | 64.01 |
| food_prices | PLAIN + ZSTD | 0.314 | 0.693 | 18.13 |
| food_prices | ALP | 0.587 | 7.385 | 23.20 |
| gov10 | PLAIN | 13.788 | 13.031 | 64.01 |
| gov10 | PLAIN + ZSTD | 0.275 | 0.699 | 29.12 |
| gov10 | ALP | 0.966 | 9.393 | 29.88 |
| gov26 | PLAIN | 13.832 | 13.086 | 64.01 |
| gov26 | PLAIN + ZSTD | 3.281 | 5.765 | 0.20 |
| gov26 | ALP | 0.924 | 18.473 | 1.40 |
| gov30 | PLAIN | 13.828 | 13.085 | 64.01 |
| gov30 | PLAIN + ZSTD | 1.112 | 2.440 | 4.52 |
| gov30 | ALP | 0.601 | 11.786 | 17.88 |
| gov31 | PLAIN | 13.809 | 13.074 | 64.01 |
| gov31 | PLAIN + ZSTD | 1.756 | 3.387 | 1.65 |
| gov31 | ALP | 1.250 | 13.075 | 6.77 |
| gov40 | PLAIN | 13.789 | 13.057 | 64.01 |
| gov40 | PLAIN + ZSTD | 2.971 | 5.021 | 0.43 |
| gov40 | ALP | 1.346 | 16.493 | 2.59 |
| medicare1 | PLAIN | 13.721 | 12.113 | 64.01 |
| medicare1 | PLAIN + ZSTD | 0.299 | 0.823 | 31.68 |
| medicare1 | ALP | 0.650 | 6.870 | 40.46 |
| medicare9 | PLAIN | 13.787 | 13.137 | 64.01 |
| medicare9 | PLAIN + ZSTD | 0.351 | 0.707 | 11.86 |
| medicare9 | ALP | 1.208 | 10.952 | 12.82 |
| neon_air_pressure | PLAIN | 13.822 | 13.118 | 64.01 |
| neon_air_pressure | PLAIN + ZSTD | 0.452 | 1.069 | 11.85 |
| neon_air_pressure | ALP | 1.199 | 10.680 | 16.48 |
| neon_bio_temp_c | PLAIN | 13.783 | 13.082 | 64.01 |
| neon_bio_temp_c | PLAIN + ZSTD | 0.314 | 0.763 | 16.84 |
| neon_bio_temp_c | ALP | 1.237 | 11.511 | 10.81 |
| neon_dew_point_temp | PLAIN | 13.855 | 12.947 | 64.01 |
| neon_dew_point_temp | PLAIN + ZSTD | 0.266 | 0.799 | 23.73 |
| neon_dew_point_temp | ALP | 1.213 | 10.294 | 13.63 |
| neon_pm10_dust | PLAIN | 13.550 | 14.790 | 64.01 |
| neon_pm10_dust | PLAIN + ZSTD | 0.471 | 0.896 | 7.79 |
| neon_pm10_dust | ALP | 0.834 | 12.082 | 8.41 |
| neon_wind_dir | PLAIN | 13.774 | 13.098 | 64.01 |
| neon_wind_dir | PLAIN + ZSTD | 0.272 | 0.743 | 24.41 |
| neon_wind_dir | ALP | 1.208 | 12.560 | 15.94 |
| nyc29 | PLAIN | 13.751 | 12.170 | 64.01 |
| nyc29 | PLAIN + ZSTD | 0.343 | 0.847 | 24.67 |
| nyc29 | ALP | 1.061 | 8.720 | 40.43 |
| poi_lat | PLAIN | 13.366 | 5.087 | 64.01 |
| poi_lat | PLAIN + ZSTD | 0.329 | 0.889 | 57.78 |
| poi_lat | ALP | 0.623 | 3.158 | 88.19 |
| poi_lon | PLAIN | 13.681 | 5.958 | 64.01 |
| poi_lon | PLAIN + ZSTD | 0.414 | 0.961 | 60.44 |
| poi_lon | ALP | 0.702 | 3.836 | 79.12 |
| ssd_hdd_benchmarks_f | PLAIN | 17.871 | 22.035 | 64.02 |
| ssd_hdd_benchmarks_f | PLAIN + ZSTD | 0.334 | 0.684 | 12.98 |
| ssd_hdd_benchmarks_f | ALP | 1.205 | 11.831 | 16.04 |
| stocks_de | PLAIN | 13.730 | 12.883 | 64.01 |
| stocks_de | PLAIN + ZSTD | 0.381 | 0.846 | 10.07 |
| stocks_de | ALP | 0.790 | 11.214 | 11.20 |
| stocks_uk | PLAIN | 13.792 | 13.182 | 64.01 |
| stocks_uk | PLAIN + ZSTD | 0.364 | 0.762 | 11.29 |
| stocks_uk | ALP | 0.546 | 11.074 | 12.75 |
| stocks_usa_c | PLAIN | 13.793 | 13.056 | 64.01 |
| stocks_usa_c | PLAIN + ZSTD | 0.410 | 0.854 | 8.24 |
| stocks_usa_c | ALP | 1.250 | 11.793 | 7.95 |
| **ALL AVG.** | **PLAIN** | **13.936** | **12.696** | **64.01** |
| **ALL AVG.** | **PLAIN + ZSTD** | **0.603** | **1.266** | **22.75** |
| **ALL AVG.** | **ALP** | **0.979** | **9.611** | **24.27** |

GB/s is decimal billions of uncompressed input bytes processed per second; higher is better. Compressed size includes Parquet data-page headers but excludes the file footer. Speed processes every value in pages of up to 131072 values and excludes file I/O. PLAIN + ZSTD includes both stages: PLAIN encoding plus ZSTD compression, and ZSTD decompression plus PLAIN decoding. Short pages are repeated for timing stability and normalized to one page.

## Random access

Time to decode 100 deterministic, uniformly distributed rows from `city_temperature_f` (lower is better). Each lookup starts from the encoded page.

| Parquet choice | 100 random rows (µs) |
|---|---:|
| PLAIN | 4.044 |
| PLAIN + ZSTD | 145706.934 |
| ALP | 16.891 |

PLAIN and ALP reset the page decoder, skip to the selected row, and decode one value. PLAIN + ZSTD additionally decompresses the complete target page for every independent lookup. Encoded pages are already in memory; file I/O and page lookup are excluded.

30 datasets. Arithmetic mean: PLAIN 64.01, PLAIN + ZSTD 22.75, ALP 24.27 bits/value.
Median ALP: 17.88 bits/value. ALP is 0.28x the size of PLAIN and 1.25x the size of PLAIN + ZSTD by geometric mean.
ALP is smaller than PLAIN + ZSTD on 10/30 datasets.
Arithmetic mean compression/decompression speed in GB/s: PLAIN 13.936/12.696, PLAIN + ZSTD 0.603/1.266, ALP 0.979/9.611.
