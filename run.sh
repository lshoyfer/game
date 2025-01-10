#!/usr/bin/env bash
# Usage: ./run.sh <log-level>
# Valid log-levels: off | error | warn | info | debug | trace
# This script calls cargo run, which means it runs in debug (not dev/release mode)

set -e

if [ "$#" -ne 1 ]; then
  echo "Error: You must provide exactly one argument for the log level."
  echo "Usage: $0 [off|error|warn|info|debug|trace]"
  exit 1
fi

LOG_LEVEL="$1"

case "$LOG_LEVEL" in
  off|error|warn|info|debug|trace)
    RUST_LOG="$LOG_LEVEL" cargo run
    ;;
  *)
    echo "Invalid log level: $LOG_LEVEL"
    echo "Usage: $0 [off|error|warn|info|debug|trace]"
    exit 1
    ;;
esac
