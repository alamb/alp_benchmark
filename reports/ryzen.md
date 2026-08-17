## Benchmark environment

| Environment | Value |
|---|---|
| UTC timestamp | `2026-08-07T22:10:51Z` |
| Commit | `0ffde54952cc6fc6ac0fd25a739b4a43022a770b` |
| Worktree | clean |
| CPU | AMD Ryzen AI 9 HX PRO 470 w/ Radeon 890M |
| Architecture | `x86_64` |
| SIMD ISA | `AVX-512F, AVX2, AVX` |
| Logical CPUs | 24 |
| OS and kernel | `Linux 6.19.10-300.fc44.x86_64` |
| CPU governor | `powersave` |
| Rust | `rustc 1.96.1 (31fca3adb 2026-06-26)` |
| LLVM | `22.1.2` |
| Cargo | `cargo 1.96.1 (356927216 2026-06-26)` |
| RUSTFLAGS | `-C target-cpu=native` |
| Dataset archive SHA-256 | `1070817918b9e2b2cc7003995927bd04fe7b942045383913d3f40437eda29831` |


## Parquet compression results

| Dataset | Parquet choice | Compression (GB/s) | Decompression (GB/s) | Compressed size (bits/value) |
|---|---|---:|---:|---:|
| arade4 | PLAIN | 74.479 | 74.591 | 64.01 |
| arade4 | PLAIN + ZSTD | 0.653 | 1.640 | 37.39 |
| arade4 | ALP | 2.662 | 34.503 | 24.99 |
| basel_temp_f | PLAIN | 67.349 | 74.978 | 64.01 |
| basel_temp_f | PLAIN + ZSTD | 0.461 | 1.655 | 23.07 |
| basel_temp_f | ALP | 1.362 | 22.070 | 29.23 |
| basel_wind_f | PLAIN | 61.719 | 74.922 | 64.01 |
| basel_wind_f | PLAIN + ZSTD | 0.583 | 1.668 | 18.53 |
| basel_wind_f | ALP | 2.444 | 26.226 | 29.87 |
| bird_migration_f | PLAIN | 62.842 | 106.453 | 64.01 |
| bird_migration_f | PLAIN + ZSTD | 0.633 | 1.774 | 23.49 |
| bird_migration_f | ALP | 2.657 | 25.455 | 20.24 |
| bitcoin_f | PLAIN | 83.667 | 113.938 | 64.07 |
| bitcoin_f | PLAIN + ZSTD | 0.570 | 1.640 | 50.01 |
| bitcoin_f | ALP | 1.789 | 29.517 | 27.18 |
| bitcoin_transactions_f | PLAIN | 64.196 | 74.489 | 64.01 |
| bitcoin_transactions_f | PLAIN + ZSTD | 1.088 | 2.000 | 47.96 |
| bitcoin_transactions_f | ALP | 2.331 | 20.391 | 41.27 |
| city_temperature_f | PLAIN | 75.474 | 76.966 | 64.01 |
| city_temperature_f | PLAIN + ZSTD | 0.568 | 1.372 | 17.67 |
| city_temperature_f | ALP | 2.788 | 34.158 | 10.80 |
| cms1 | PLAIN | 76.093 | 30.520 | 64.01 |
| cms1 | PLAIN + ZSTD | 0.666 | 1.526 | 26.84 |
| cms1 | ALP | 1.411 | 14.770 | 35.19 |
| cms25 | PLAIN | 71.629 | 73.437 | 64.01 |
| cms25 | PLAIN + ZSTD | 0.834 | 1.841 | 58.11 |
| cms25 | ALP | 2.202 | 24.216 | 41.17 |
| cms9 | PLAIN | 76.616 | 77.507 | 64.01 |
| cms9 | PLAIN + ZSTD | 0.719 | 1.471 | 11.71 |
| cms9 | ALP | 2.803 | 33.547 | 12.16 |
| food_prices | PLAIN | 68.581 | 74.763 | 64.01 |
| food_prices | PLAIN + ZSTD | 0.580 | 1.353 | 18.13 |
| food_prices | ALP | 1.154 | 20.379 | 23.20 |
| gov10 | PLAIN | 75.242 | 76.877 | 64.01 |
| gov10 | PLAIN + ZSTD | 0.518 | 1.260 | 29.12 |
| gov10 | ALP | 1.783 | 26.573 | 29.88 |
| gov26 | PLAIN | 75.922 | 76.636 | 64.01 |
| gov26 | PLAIN + ZSTD | 12.506 | 25.474 | 0.20 |
| gov26 | ALP | 2.107 | 94.626 | 1.40 |
| gov30 | PLAIN | 75.812 | 76.574 | 64.01 |
| gov30 | PLAIN + ZSTD | 2.203 | 5.277 | 4.52 |
| gov30 | ALP | 1.205 | 39.760 | 17.88 |
| gov31 | PLAIN | 64.361 | 65.519 | 64.01 |
| gov31 | PLAIN + ZSTD | 3.542 | 8.255 | 1.65 |
| gov31 | ALP | 2.551 | 40.989 | 6.77 |
| gov40 | PLAIN | 57.109 | 59.596 | 64.01 |
| gov40 | PLAIN + ZSTD | 8.097 | 14.910 | 0.43 |
| gov40 | ALP | 2.710 | 60.151 | 2.59 |
| medicare1 | PLAIN | 60.313 | 58.146 | 64.01 |
| medicare1 | PLAIN + ZSTD | 0.519 | 1.364 | 31.68 |
| medicare1 | ALP | 1.218 | 14.356 | 40.46 |
| medicare9 | PLAIN | 62.343 | 63.324 | 64.01 |
| medicare9 | PLAIN + ZSTD | 0.644 | 1.314 | 11.86 |
| medicare9 | ALP | 2.497 | 27.809 | 12.82 |
| neon_air_pressure | PLAIN | 73.548 | 74.262 | 64.01 |
| neon_air_pressure | PLAIN + ZSTD | 0.805 | 2.029 | 11.85 |
| neon_air_pressure | ALP | 2.692 | 34.409 | 16.48 |
| neon_bio_temp_c | PLAIN | 74.851 | 75.700 | 64.01 |
| neon_bio_temp_c | PLAIN + ZSTD | 0.563 | 1.559 | 16.84 |
| neon_bio_temp_c | ALP | 2.776 | 33.096 | 10.81 |
| neon_dew_point_temp | PLAIN | 73.178 | 73.636 | 64.01 |
| neon_dew_point_temp | PLAIN + ZSTD | 0.478 | 1.651 | 23.73 |
| neon_dew_point_temp | ALP | 2.728 | 30.639 | 13.63 |
| neon_pm10_dust | PLAIN | 47.279 | 70.301 | 64.01 |
| neon_pm10_dust | PLAIN + ZSTD | 0.848 | 1.663 | 7.79 |
| neon_pm10_dust | ALP | 1.794 | 34.353 | 8.41 |
| neon_wind_dir | PLAIN | 73.187 | 74.370 | 64.01 |
| neon_wind_dir | PLAIN + ZSTD | 0.493 | 1.486 | 24.41 |
| neon_wind_dir | ALP | 2.689 | 46.726 | 15.94 |
| nyc29 | PLAIN | 72.889 | 70.456 | 64.01 |
| nyc29 | PLAIN + ZSTD | 0.615 | 1.483 | 24.67 |
| nyc29 | ALP | 2.434 | 24.030 | 40.43 |
| poi_lat | PLAIN | 72.573 | 18.951 | 64.01 |
| poi_lat | PLAIN + ZSTD | 0.682 | 1.533 | 57.78 |
| poi_lat | ALP | 1.522 | 10.882 | 88.19 |
| poi_lon | PLAIN | 73.593 | 19.404 | 64.01 |
| poi_lon | PLAIN + ZSTD | 0.854 | 1.774 | 60.44 |
| poi_lon | ALP | 1.713 | 16.118 | 79.12 |
| ssd_hdd_benchmarks_f | PLAIN | 81.743 | 113.024 | 64.02 |
| ssd_hdd_benchmarks_f | PLAIN + ZSTD | 0.800 | 1.772 | 12.98 |
| ssd_hdd_benchmarks_f | ALP | 2.662 | 34.116 | 16.04 |
| stocks_de | PLAIN | 73.346 | 74.265 | 64.01 |
| stocks_de | PLAIN + ZSTD | 0.684 | 1.662 | 10.07 |
| stocks_de | ALP | 1.496 | 33.555 | 11.20 |
| stocks_uk | PLAIN | 74.759 | 76.045 | 64.01 |
| stocks_uk | PLAIN + ZSTD | 0.673 | 1.491 | 11.29 |
| stocks_uk | ALP | 0.936 | 35.561 | 12.75 |
| stocks_usa_c | PLAIN | 71.756 | 73.168 | 64.01 |
| stocks_usa_c | PLAIN + ZSTD | 0.717 | 1.585 | 8.24 |
| stocks_usa_c | ALP | 2.714 | 35.625 | 7.95 |
| **ALL AVG.** | **PLAIN** | **70.548** | **71.427** | **64.01** |
| **ALL AVG.** | **PLAIN + ZSTD** | **1.453** | **3.183** | **22.75** |
| **ALL AVG.** | **ALP** | **2.128** | **31.954** | **24.27** |

GB/s is decimal billions of uncompressed input bytes processed per second; higher is better. Compressed size includes Parquet data-page headers but excludes the file footer. Speed processes every value in pages of up to 131072 values and excludes file I/O. PLAIN + ZSTD includes both stages: PLAIN encoding plus ZSTD compression, and ZSTD decompression plus PLAIN decoding. Short pages are repeated for timing stability and normalized to one page.

## Random access

Time to decode 100 deterministic, uniformly distributed rows from `city_temperature_f` (lower is better). Each lookup starts from the encoded page.

| Parquet choice | 100 random rows (µs) |
|---|---:|
| PLAIN | 2.817 |
| PLAIN + ZSTD | 75898.583 |
| ALP | 10.100 |

PLAIN and ALP reset the page decoder, skip to the selected row, and decode one value. PLAIN + ZSTD additionally decompresses the complete target page for every independent lookup. Encoded pages are already in memory; file I/O and page lookup are excluded.

30 datasets. Arithmetic mean: PLAIN 64.01, PLAIN + ZSTD 22.75, ALP 24.27 bits/value.
Median ALP: 17.88 bits/value. ALP is 0.28x the size of PLAIN and 1.25x the size of PLAIN + ZSTD by geometric mean.
ALP is smaller than PLAIN + ZSTD on 10/30 datasets.
Arithmetic mean compression/decompression speed in GB/s: PLAIN 70.548/71.427, PLAIN + ZSTD 1.453/3.183, ALP 2.128/31.954.
