use crate::compressor::decompress_text;
use crate::dictionary::DictionaryStore;

pub fn decompress(text: &str, store: &DictionaryStore) -> String {
    decompress_text(text, store)
}
