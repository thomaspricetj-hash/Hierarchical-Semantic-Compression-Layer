use crate::dictionary::DictionaryStore;
use std::collections::HashMap;

pub fn build_dictionary(text: &str, min_occurrences: usize) -> DictionaryStore {
    let mut phrase_counts: HashMap<String, usize> = HashMap::new();
    let words: Vec<&str> = text.split_whitespace().collect();
    
    for n in 2..=5 {
        for i in 0..words.len().saturating_sub(n-1) {
            let phrase = words[i..i+n].join(" ");
            *phrase_counts.entry(phrase).or_insert(0) += 1;
        }
    }

    let mut store = DictionaryStore::new();
    let mut id = 4000;
    for (phrase, count) in phrase_counts {
        if count >= min_occurrences && phrase.split_whitespace().count() >= 2 {
            id += 1;
            store.insert(id, phrase);
        }
    }
    store
}

pub fn compress_text(text: &str, store: &DictionaryStore) -> String {
    let mut result = text.to_string();
    // Replace longer phrases first
    let mut ids: Vec<u32> = store.dict.keys().cloned().collect();
    ids.sort_by(|a, b| {
        let len_a = store.dict.get(a).map(|s| s.len()).unwrap_or(0);
        let len_b = store.dict.get(b).map(|s| s.len()).unwrap_or(0);
        len_b.cmp(&len_a)
    });
    
    for id in ids {
        if let Some(phrase) = store.dict.get(&id) {
            let replacement = id.to_string();
            result = result.replace(phrase, &replacement);
        }
    }
    result
}

pub fn decompress_text(text: &str, store: &DictionaryStore) -> String {
    let mut result = text.to_string();
    let mut ids: Vec<u32> = store.dict.keys().cloned().collect();
    ids.sort_unstable();
    
    for id in ids {
        if let Some(phrase) = store.dict.get(&id) {
            let replacement = id.to_string();
            result = result.replace(&replacement, phrase);
        }
    }
    result
}
