use std::collections::HashMap;

pub type Dictionary = HashMap<u32, String>;
pub type ReverseDictionary = HashMap<String, u32>;

pub struct DictionaryStore {
    pub dict: Dictionary,
    pub reverse: ReverseDictionary,
}

impl DictionaryStore {
    pub fn new() -> Self {
        Self { dict: HashMap::new(), reverse: HashMap::new() }
    }

    pub fn insert(&mut self, id: u32, phrase: String) {
        self.reverse.insert(phrase.clone(), id);
        self.dict.insert(id, phrase);
    }

    pub fn get_id(&self, phrase: &str) -> Option<u32> {
        self.reverse.get(phrase).copied()
    }

    pub fn get_phrase(&self, id: u32) -> Option<&String> {
        self.dict.get(&id)
    }
}
