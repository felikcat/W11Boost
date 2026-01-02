#!/bin/bash
# Note: Tests require administrator privileges to run (registry modifications)
# Run this script from an elevated terminal

export PATH="/c/msys64/mingw64/bin:$PATH"
cargo +nightly test \
  --target x86_64-pc-windows-gnu \
  --features fltk/fltk-bundled \
  "$@"
