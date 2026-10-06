# HSCL Prototype

Hierarchical Semantic Compression Layer proof-of-concept in Rust.

## Build
```bash
cargo +stable-x86_64-pc-windows-gnu build --release
```

## Usage
```bash
cargo +stable-x86_64-pc-windows-gnu run --release -- build corpora/small_synthetic.txt
cargo +stable-x86_64-pc-windows-gnu run --release -- compress corpora/small_synthetic.txt
cargo +stable-x86_64-pc-windows-gnu run --release -- decompress compressed.txt
cargo +stable-x86_64-pc-windows-gnu run --release -- benchmark corpora/small_synthetic.txt
```

## Results
See REPORT.md

All corpora show token reduction with lossless reconstruction.

## Structure
- src/main.rs
- src/compressor.rs
- src/dictionary.rs
- src/metrics.rs
- src/tokenizer.rs
- src/decompressor.rs
- src/lib.rs
