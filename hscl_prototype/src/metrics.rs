pub struct Metrics {
    pub original_chars: usize,
    pub original_words: usize,
    pub original_tokens: usize,
    pub compressed_chars: usize,
    pub compressed_words: usize,
    pub compressed_tokens: usize,
}

impl Metrics {
    pub fn reduction_percent(&self) -> f64 {
        if self.original_tokens == 0 {
            return 0.0;
        }
        100.0 * (self.original_tokens as f64 - self.compressed_tokens as f64) / self.original_tokens as f64
    }
}

pub fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

pub fn count_chars(text: &str) -> usize {
    text.chars().count()
}

pub fn token_count(text: &str) -> usize {
    // Simple whitespace tokenizer for PoC
    // In production use a real tokenizer like tiktoken
    text.split_whitespace().count()
}
