#!/bin/bash
export PATH="/c/msys64/mingw64/bin:$PATH"
export RUSTFLAGS="-Zlocation-detail=none -Zfmt-debug=none -Zunstable-options -Cpanic=immediate-abort"
cargo +nightly build \
  -Z build-std=std,panic_abort \
  -Z build-std-features="optimize_for_size" \
  --target x86_64-pc-windows-gnu --release \
  --features fltk/fltk-bundled
