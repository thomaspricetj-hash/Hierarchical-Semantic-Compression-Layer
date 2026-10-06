use std::fs;
use std::collections::HashMap;
use hscl::{compressor, dictionary::DictionaryStore, metrics::{Metrics, count_chars, count_words, token_count}};

fn dict_to_json(dict: &HashMap<u32, String>) -> String {
    let mut s = String::from("{");
    let mut first = true;
    for (id, phrase) in dict {
        if !first { s.push(','); }
        first = false;
        // escape quotes in phrase
        let esc = phrase.replace("\\", "\\\\").replace("\"", "\\\"");
        s.push_str(&format!("\"{}\":\"{}\"", id, esc));
    }
    s.push('}');
    s
}

fn parse_dict_json(s: &str) -> HashMap<u32, String> {
    let mut map = HashMap::new();
    let s = s.trim();
    if !s.starts_with('{') || !s.ends_with('}') { return map; }
    let inner = &s[1..s.len()-1];
    for pair in inner.split(',') {
        let parts: Vec<&str> = pair.splitn(2, ':').collect();
        if parts.len() != 2 { continue; }
        let key = parts[0].trim().trim_matches('"');
        let val_raw = parts[1].trim().trim_matches('"');
        // unescape simple
        let val = val_raw.replace("\\\"", "\"").replace("\\\\", "\\");
        if let Ok(id) = key.parse::<u32>() {
            map.insert(id, val);
        }
    }
    map
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: hscl <command> <file>");
        eprintln!("Commands: build, compress, decompress, benchmark");
        std::process::exit(1);
    }
    let command = args[1].as_str();
    let file = args[2].as_str();
    match command {
        "build" => {
            let text = fs::read_to_string(file).expect("Failed to read corpus");
            let store = compressor::build_dictionary(&text, 2);
            let json = dict_to_json(&store.dict);
            fs::write("dictionary.json", json).expect("Failed to write dictionary");
            println!("Dictionary built with {} entries", store.dict.len());
        }
        "compress" => {
            let text = fs::read_to_string(file).expect("Failed to read corpus");
            let dict_json = fs::read_to_string("dictionary.json").expect("Dictionary not found");
            let dict_map = parse_dict_json(&dict_json);
            let mut store = DictionaryStore::new();
            for (id, phrase) in dict_map {
                store.insert(id, phrase);
            }
            let compressed = compressor::compress_text(&text, &store);
            fs::write("compressed.txt", &compressed).expect("Failed to write compressed");
            println!("Compressed written to compressed.txt");
        }
        "decompress" => {
            let text = fs::read_to_string(file).expect("Failed to read input");
            let dict_json = fs::read_to_string("dictionary.json").expect("Dictionary not found");
            let dict_map = parse_dict_json(&dict_json);
            let mut store = DictionaryStore::new();
            for (id, phrase) in dict_map {
                store.insert(id, phrase);
            }
            let decompressed = compressor::decompress_text(&text, &store);
            fs::write("decompressed.txt", &decompressed).expect("Failed to write decompressed");
            println!("Decompressed written to decompressed.txt");
        }
        "benchmark" => {
            let text = fs::read_to_string(file).expect("Failed to read corpus");
            let store = compressor::build_dictionary(&text, 2);
            let compressed = compressor::compress_text(&text, &store);
            let decompressed = compressor::decompress_text(&compressed, &store);
            
            if text != decompressed {
                eprintln!("Reconstruction failed!");
                std::process::exit(1);
            }
            
            let original_tokens = token_count(&text);
            let compressed_tokens = token_count(&compressed);
            
            let metrics = Metrics {
                original_chars: count_chars(&text),
                original_words: count_words(&text),
                original_tokens,
                compressed_chars: count_chars(&compressed),
                compressed_words: count_words(&compressed),
                compressed_tokens,
            };
            
            let reduction = metrics.reduction_percent();
            let results = format!(
                "{{\"original_tokens\":{},\"compressed_tokens\":{},\"reduction_percent\":{:.4},\"original_chars\":{},\"compressed_chars\":{},\"dictionary_size\":{},\"reconstruction_valid\":true}}",
                metrics.original_tokens,
                metrics.compressed_tokens,
                reduction,
                metrics.original_chars,
                metrics.compressed_chars,
                store.dict.len()
            );
            
            fs::write("results.json", results).expect("Failed to write results");
            println!("Benchmark complete");
            println!("Original tokens: {}", metrics.original_tokens);
            println!("Compressed tokens: {}", metrics.compressed_tokens);
            println!("Reduction: {:.2}%", reduction);
            println!("Dictionary size: {}", store.dict.len());
        }
        _ => {
            eprintln!("Unknown command");
            std::process::exit(1);
        }
    }
}
