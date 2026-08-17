#!/usr/bin/env bash

# Licensed to the Apache Software Foundation (ASF) under one
# or more contributor license agreements.  See the NOTICE file
# distributed with this work for additional information
# regarding copyright ownership.  The ASF licenses this file
# to you under the Apache License, Version 2.0 (the
# "License"); you may not use this file except in compliance
# with the License.  You may obtain a copy of the License at
# http://www.apache.org/licenses/LICENSE-2.0

# Ported from parquet/examples/alp_to_parquet.sh in
# https://github.com/apache/arrow-rs/pull/10696
#
# Converts the raw CWI .bin datasets in ./data (or ALP_DATASET_DIR) into
# one-column Parquet files in the given output directory. Run ./benchmark.sh
# first to download the datasets.

set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "usage: $0 <output-dir>" >&2
  exit 2
fi

REPO_ROOT="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
INPUT_DIR="${ALP_DATASET_DIR:-${REPO_ROOT}/data}"

cd "$REPO_ROOT"
export RUSTFLAGS="${RUSTFLAGS:--C target-cpu=native}"
exec cargo run --release --bin bin_to_parquet -- "$INPUT_DIR" "$1"
