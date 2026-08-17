## Benchmark environment

| Environment | Value |
|---|---|
| UTC timestamp | `2026-08-18T12:08:22Z` |
| Commit | `814810bd1d4d800ceec3ec773326e544d90fa6fe` |
| Worktree | dirty |
| Parquet rev (apache/arrow-rs PR 9372) | `f9794b4f4ac9fa896ed507b6d7c6e7556db041a9` |
| CPU | Apple M3 Max |
| Architecture | `arm64` |
| SIMD ISA | `NEON` |
| Logical CPUs | 16 |
| OS and kernel | `Darwin 25.3.0` |
| CPU governor | `unavailable` |
| Rust | `rustc 1.97.1 (8bab26f4f 2026-07-14)` |
| LLVM | `22.1.6` |
| Cargo | `cargo 1.97.1 (c980f4866 2026-06-30)` |
| RUSTFLAGS | `-C target-cpu=native` |
| Dataset archive SHA-256 | `1070817918b9e2b2cc7003995927bd04fe7b942045383913d3f40437eda29831` |


## Parquet compression results

| Dataset | Parquet choice | Compression (GB/s) | Decompression (GB/s) | Compressed size (bits/value) |
|---|---|---:|---:|---:|
| arade4 | PLAIN | 75.634 | 78.455 | 64.01 |
| arade4 | PLAIN + ZSTD | 0.662 | 1.347 | 37.39 |
| arade4 | ALP | 3.161 | 24.500 | 24.99 |
| basel_temp_f | PLAIN | 78.118 | 81.054 | 64.01 |
| basel_temp_f | PLAIN + ZSTD | 0.430 | 1.454 | 23.07 |
| basel_temp_f | ALP | 1.691 | 23.948 | 29.23 |
| basel_wind_f | PLAIN | 76.974 | 82.609 | 64.01 |
| basel_wind_f | PLAIN + ZSTD | 0.515 | 1.559 | 18.53 |
| basel_wind_f | ALP | 2.991 | 24.442 | 29.87 |
| bird_migration_f | PLAIN | 62.997 | 82.368 | 64.01 |
| bird_migration_f | PLAIN + ZSTD | 0.573 | 1.517 | 23.49 |
| bird_migration_f | ALP | 3.276 | 13.266 | 20.24 |
| bitcoin_f | PLAIN | 71.239 | 75.264 | 64.07 |
| bitcoin_f | PLAIN + ZSTD | 0.548 | 1.363 | 50.01 |
| bitcoin_f | ALP | 2.322 | 24.084 | 27.18 |
| bitcoin_transactions_f | PLAIN | 71.892 | 75.182 | 64.01 |
| bitcoin_transactions_f | PLAIN + ZSTD | 0.973 | 1.430 | 47.96 |
| bitcoin_transactions_f | ALP | 2.558 | 16.171 | 41.27 |
| city_temperature_f | PLAIN | 73.625 | 74.485 | 64.01 |
| city_temperature_f | PLAIN + ZSTD | 0.489 | 1.311 | 17.67 |
| city_temperature_f | ALP | 3.560 | 26.593 | 10.80 |
| cms1 | PLAIN | 73.201 | 75.830 | 64.01 |
| cms1 | PLAIN + ZSTD | 0.664 | 1.529 | 26.84 |
| cms1 | ALP | 1.641 | 17.924 | 35.19 |
| cms25 | PLAIN | 75.195 | 77.707 | 64.01 |
| cms25 | PLAIN + ZSTD | 0.810 | 1.428 | 58.11 |
| cms25 | ALP | 2.636 | 21.716 | 41.17 |
| cms9 | PLAIN | 75.252 | 77.075 | 64.01 |
| cms9 | PLAIN + ZSTD | 0.606 | 1.355 | 11.71 |
| cms9 | ALP | 3.536 | 26.561 | 12.16 |
| food_prices | PLAIN | 76.111 | 78.745 | 64.01 |
| food_prices | PLAIN + ZSTD | 0.539 | 1.324 | 18.13 |
| food_prices | ALP | 1.376 | 18.069 | 23.20 |
| gov10 | PLAIN | 75.126 | 78.174 | 64.01 |
| gov10 | PLAIN + ZSTD | 0.523 | 1.151 | 29.12 |
| gov10 | ALP | 2.953 | 24.572 | 29.88 |
| gov26 | PLAIN | 73.793 | 75.607 | 64.01 |
| gov26 | PLAIN + ZSTD | 9.979 | 9.816 | 0.20 |
| gov26 | ALP | 2.927 | 62.126 | 1.40 |
| gov30 | PLAIN | 74.656 | 76.348 | 64.01 |
| gov30 | PLAIN + ZSTD | 2.166 | 3.840 | 4.52 |
| gov30 | ALP | 1.619 | 31.466 | 17.88 |
| gov31 | PLAIN | 74.001 | 76.874 | 64.01 |
| gov31 | PLAIN + ZSTD | 3.679 | 5.212 | 1.65 |
| gov31 | ALP | 3.994 | 39.748 | 6.77 |
| gov40 | PLAIN | 73.730 | 75.423 | 64.01 |
| gov40 | PLAIN + ZSTD | 8.407 | 8.756 | 0.43 |
| gov40 | ALP | 4.369 | 56.513 | 2.59 |
| medicare1 | PLAIN | 75.193 | 69.620 | 64.01 |
| medicare1 | PLAIN + ZSTD | 0.570 | 1.398 | 31.68 |
| medicare1 | ALP | 1.545 | 17.259 | 40.46 |
| medicare9 | PLAIN | 73.243 | 74.152 | 64.01 |
| medicare9 | PLAIN + ZSTD | 0.594 | 1.334 | 11.86 |
| medicare9 | ALP | 3.489 | 25.340 | 12.82 |
| neon_air_pressure | PLAIN | 75.368 | 77.241 | 64.01 |
| neon_air_pressure | PLAIN + ZSTD | 0.754 | 2.042 | 11.85 |
| neon_air_pressure | ALP | 3.427 | 26.048 | 16.48 |
| neon_bio_temp_c | PLAIN | 74.556 | 78.401 | 64.01 |
| neon_bio_temp_c | PLAIN + ZSTD | 0.500 | 1.473 | 16.84 |
| neon_bio_temp_c | ALP | 3.590 | 27.034 | 10.81 |
| neon_dew_point_temp | PLAIN | 75.041 | 77.015 | 64.01 |
| neon_dew_point_temp | PLAIN + ZSTD | 0.441 | 1.468 | 23.73 |
| neon_dew_point_temp | ALP | 3.493 | 24.627 | 13.63 |
| neon_pm10_dust | PLAIN | 73.156 | 79.740 | 64.01 |
| neon_pm10_dust | PLAIN + ZSTD | 0.766 | 1.760 | 7.79 |
| neon_pm10_dust | ALP | 2.362 | 26.605 | 8.41 |
| neon_wind_dir | PLAIN | 75.608 | 79.690 | 64.01 |
| neon_wind_dir | PLAIN + ZSTD | 0.461 | 1.351 | 24.41 |
| neon_wind_dir | ALP | 3.516 | 29.702 | 15.94 |
| nyc29 | PLAIN | 75.936 | 73.297 | 64.01 |
| nyc29 | PLAIN + ZSTD | 0.616 | 1.478 | 24.67 |
| nyc29 | ALP | 2.646 | 22.133 | 40.43 |
| poi_lat | PLAIN | 75.443 | 34.480 | 64.01 |
| poi_lat | PLAIN + ZSTD | 0.708 | 1.385 | 57.78 |
| poi_lat | ALP | 1.760 | 10.429 | 88.19 |
| poi_lon | PLAIN | 76.033 | 78.540 | 64.01 |
| poi_lon | PLAIN + ZSTD | 0.849 | 1.450 | 60.44 |
| poi_lon | ALP | 1.927 | 13.028 | 79.12 |
| ssd_hdd_benchmarks_f | PLAIN | 55.893 | 95.221 | 64.02 |
| ssd_hdd_benchmarks_f | PLAIN + ZSTD | 0.640 | 1.391 | 12.98 |
| ssd_hdd_benchmarks_f | ALP | 3.233 | 23.894 | 16.04 |
| stocks_de | PLAIN | 75.484 | 76.786 | 64.01 |
| stocks_de | PLAIN + ZSTD | 0.608 | 1.642 | 10.07 |
| stocks_de | ALP | 1.887 | 26.475 | 11.20 |
| stocks_uk | PLAIN | 75.420 | 76.801 | 64.01 |
| stocks_uk | PLAIN + ZSTD | 0.602 | 1.521 | 11.29 |
| stocks_uk | ALP | 1.066 | 26.974 | 12.75 |
| stocks_usa_c | PLAIN | 75.358 | 78.440 | 64.01 |
| stocks_usa_c | PLAIN + ZSTD | 0.662 | 1.659 | 8.24 |
| stocks_usa_c | ALP | 3.674 | 28.012 | 7.95 |
| **ALL AVG.** | **PLAIN** | **73.776** | **76.354** | **64.01** |
| **ALL AVG.** | **PLAIN + ZSTD** | **1.345** | **2.191** | **22.75** |
| **ALL AVG.** | **ALP** | **2.741** | **25.975** | **24.27** |

GB/s is decimal billions of uncompressed input bytes processed per second; higher is better. Compressed size includes Parquet data-page headers but excludes the file footer. Speed processes every value in pages of up to 131072 values and excludes file I/O. PLAIN + ZSTD includes both stages: PLAIN encoding plus ZSTD compression, and ZSTD decompression plus PLAIN decoding. Short pages are repeated for timing stability and normalized to one page.

## Random access

Time to decode 100 deterministic, uniformly distributed rows from `city_temperature_f` (lower is better). Each lookup starts from the encoded page.

| Parquet choice | 100 random rows (µs) |
|---|---:|
| PLAIN | 0.927 |
| PLAIN + ZSTD | 75614.177 |
| ALP | 15.854 |

PLAIN and ALP reset the page decoder, skip to the selected row, and decode one value. PLAIN + ZSTD additionally decompresses the complete target page for every independent lookup. Encoded pages are already in memory; file I/O and page lookup are excluded.

30 datasets. Arithmetic mean: PLAIN 64.01, PLAIN + ZSTD 22.75, ALP 24.27 bits/value.
Median ALP: 17.88 bits/value. ALP is 0.28x the size of PLAIN and 1.25x the size of PLAIN + ZSTD by geometric mean.
ALP is smaller than PLAIN + ZSTD on 10/30 datasets.
Arithmetic mean compression/decompression speed in GB/s: PLAIN 73.776/76.354, PLAIN + ZSTD 1.345/2.191, ALP 2.741/25.975.
