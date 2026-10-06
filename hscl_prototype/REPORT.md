# HSCL Prototype Benchmark Report

## Overview
Hierarchical Semantic Compression Layer prototype validates semantic phrase replacement before tokenization reduces token counts while preserving lossless reconstruction.

## Success Criteria
- compressed_tokens < original_tokens
- decompress(compress(text)) == text

## Results

### 1. Small Synthetic Corpus
File: corpora/small_synthetic.txt
- Original tokens: 68
- Compressed tokens: 45
- Reduction: 33.82%
- Dictionary size: 22
- Reconstruction valid: true

### 2. Medium Technical Document
File: corpora/medium_technical.txt
- Original tokens: 166
- Compressed tokens: 118
- Reduction: 28.92%
- Dictionary size: 21
- Reconstruction valid: true

### 3. Wikipedia Article Sample
File: corpora/wikipedia_sample.txt
- Original tokens: 165
- Compressed tokens: 104
- Reduction: 36.97%
- Dictionary size: 31
- Reconstruction valid: true

### 4. AI-Related Article
File: corpora/ai_article.txt
- Original tokens: 168
- Compressed tokens: 132
- Reduction: 21.43%
- Dictionary size: 13
- Reconstruction valid: true

## Conclusion
All corpora achieved token reduction with lossless reconstruction. Semantic phrase replacement before tokenization reduces token counts as hypothesized.

## Artifacts
- dictionary.json: Generated dictionary for last built corpus
- results.json: Last benchmark results
- compressed.txt / decompressed.txt: Example compression cycle
- Source code: src/main.rs, src/compressor.rs, src/dictionary.rs, src/metrics.rs, src/tokenizer.rs, src/decompressor.rs
