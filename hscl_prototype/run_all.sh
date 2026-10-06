#!/bin/bash
set -e
cargo +stable-x86_64-pc-windows-gnu run --release -- benchmark corpora/small_synthetic.txt
cp results.json results_small.json
cargo +stable-x86_64-pc-windows-gnu run --release -- benchmark corpora/medium_technical.txt
cp results.json results_medium.json
cargo +stable-x86_64-pc-windows-gnu run --release -- benchmark corpora/wikipedia_sample.txt
cp results.json results_wiki.json
cargo +stable-x86_64-pc-windows-gnu run --release -- benchmark corpora/ai_article.txt
cp results.json results_ai.json
