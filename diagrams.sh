#!/bin/sh
# Generate diagrams from a benchmark report; dependencies are managed by uv.
exec uv run --script "$(dirname "$0")/diagrams.py" "$@"
