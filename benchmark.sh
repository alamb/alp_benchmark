#!/usr/bin/env bash

# Licensed to the Apache Software Foundation (ASF) under one
# or more contributor license agreements.  See the NOTICE file
# distributed with this work for additional information
# regarding copyright ownership.  The ASF licenses this file
# to you under the Apache License, Version 2.0 (the
# "License"); you may not use this file except in compliance
# with the License.  You may obtain a copy of the License at
#
#   http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing,
# software distributed under the License is distributed on an
# "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
# KIND, either express or implied.  See the License for the
# specific language governing permissions and limitations
# under the License.

# Ported from the ALP compression statistics example in
# https://github.com/apache/arrow-rs/pull/10696
#
# Runs the ALP compression benchmark and prints a markdown report. The CWI ALP
# corpus is fetched by ./download_data.sh into ./data (gitignored, cached
# between runs). Set ALP_DATASET_DIR to use a different dataset directory.

set -euo pipefail

# The commit of the ALP encoding PR https://github.com/apache/arrow-rs/pull/9372
# that the parquet dependency in Cargo.toml is pinned to.
readonly PARQUET_REV="f9794b4f4ac9fa896ed507b6d7c6e7556db041a9"

REPO_ROOT="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
DATA_DIR="${ALP_DATASET_DIR:-${REPO_ROOT}/data}"

# Single source of truth for the archive checksum is download_data.sh
ARCHIVE_SHA256="$(sed -n 's/^readonly ARCHIVE_SHA256="\([0-9a-f]*\)".*/\1/p' \
  "${REPO_ROOT}/download_data.sh")"

markdown_escape() {
  local value="$1"
  value="${value//\\/\\\\}"
  value="${value//|/\\|}"
  printf '%s' "$value"
}

cpu_model() {
  local model=""

  if [[ -r /proc/cpuinfo ]]; then
    model="$(awk -F ': *' '/^model name[[:space:]]*:/{print $2; exit}' /proc/cpuinfo)"
  fi
  if [[ -z "$model" ]] && command -v sysctl >/dev/null 2>&1; then
    model="$(sysctl -n machdep.cpu.brand_string 2>/dev/null || true)"
  fi
  if [[ -z "$model" ]] && command -v sysctl >/dev/null 2>&1; then
    model="$(sysctl -n hw.model 2>/dev/null || true)"
  fi
  printf '%s' "${model:-unknown}"
}

cpu_simd() {
  local architecture features=""

  architecture="$(uname -m)"
  if [[ -r /proc/cpuinfo ]]; then
    features="$(awk -F ': *' '/^(flags|Features)[[:space:]]*:/{print $2; exit}' /proc/cpuinfo)"
  elif command -v sysctl >/dev/null 2>&1; then
    features="$(
      sysctl -n machdep.cpu.features machdep.cpu.leaf7_features \
        2>/dev/null || true
    )"
  fi
  features="$(printf '%s' "$features" | tr '[:upper:]' '[:lower:]')"

  case "$architecture" in
    x86_64 | amd64 | i386 | i686)
      if [[ " $features " == *" avx512f "* ]]; then
        printf '%s' "AVX-512F, AVX2, AVX"
      elif [[ " $features " == *" avx2 "* ]]; then
        printf '%s' "AVX2, AVX"
      elif [[ " $features " == *" avx "* ]]; then
        printf '%s' "AVX"
      else
        printf '%s' "no AVX"
      fi
      ;;
    aarch64 | arm64 | arm*)
      if [[ " $features " == *" sve2 "* ]]; then
        printf '%s' "SVE2, SVE, NEON"
      elif [[ " $features " == *" sve "* ]]; then
        printf '%s' "SVE, NEON"
      elif [[ " $features " == *" asimd "* ]] ||
        [[ " $features " == *" neon "* ]] || [[ "$(uname -s)" == "Darwin" ]]; then
        printf '%s' "NEON"
      else
        printf '%s' "unknown"
      fi
      ;;
    *)
      printf '%s' "unknown"
      ;;
  esac
}

logical_cpus() {
  local count=""

  if command -v getconf >/dev/null 2>&1; then
    count="$(getconf _NPROCESSORS_ONLN 2>/dev/null || true)"
  fi
  if [[ -z "$count" ]] && command -v nproc >/dev/null 2>&1; then
    count="$(nproc 2>/dev/null || true)"
  fi
  printf '%s' "${count:-unknown}"
}

cpu_governor() {
  local governor_file="/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor"
  if [[ -r "$governor_file" ]]; then
    tr -d '\n' < "$governor_file"
  else
    printf '%s' "unavailable"
  fi
}

safe_rustflags() {
  if [[ "$RUSTFLAGS" =~ ^[-A-Za-z0-9_=+.,[:space:]]+$ ]]; then
    printf '%s' "$RUSTFLAGS"
  else
    printf '%s' "set; value omitted because it contains paths or shell characters"
  fi
}

print_environment() {
  local commit worktree llvm_version

  commit="$(git rev-parse HEAD)"
  if [[ -n "$(git status --porcelain --untracked-files=normal)" ]]; then
    worktree="dirty"
  else
    worktree="clean"
  fi
  llvm_version="$(rustc --version --verbose | awk -F ': *' '$1 == "LLVM version" {print $2}')"

  printf '## Benchmark environment\n\n'
  printf '| Environment | Value |\n'
  printf '|---|---|\n'
  printf '| UTC timestamp | `%s` |\n' "$(date -u '+%Y-%m-%dT%H:%M:%SZ')"
  printf '| Commit | `%s` |\n' "$commit"
  printf '| Worktree | %s |\n' "$worktree"
  printf '| Parquet rev (apache/arrow-rs PR 9372) | `%s` |\n' "$PARQUET_REV"
  printf '| CPU | %s |\n' "$(markdown_escape "$(cpu_model)")"
  printf '| Architecture | `%s` |\n' "$(uname -m)"
  printf '| SIMD ISA | `%s` |\n' "$(cpu_simd)"
  printf '| Logical CPUs | %s |\n' "$(logical_cpus)"
  printf '| OS and kernel | `%s %s` |\n' "$(uname -s)" "$(uname -r)"
  printf '| CPU governor | `%s` |\n' "$(cpu_governor)"
  printf '| Rust | `%s` |\n' "$(markdown_escape "$(rustc --version)")"
  printf '| LLVM | `%s` |\n' "$(markdown_escape "${llvm_version:-unknown}")"
  printf '| Cargo | `%s` |\n' "$(markdown_escape "$(cargo --version)")"
  printf '| RUSTFLAGS | `%s` |\n' "$(markdown_escape "$(safe_rustflags)")"
  printf '| Dataset archive SHA-256 | `%s` |\n\n' "$ARCHIVE_SHA256"
}

"${REPO_ROOT}/download_data.sh"

echo "Running the ALP compression benchmark" >&2
cd "$REPO_ROOT"
export RUSTFLAGS="${RUSTFLAGS:--C target-cpu=native}"
print_environment
exec cargo run --release --bin benchmark -- "$DATA_DIR"
