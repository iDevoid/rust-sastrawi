use crate::dictionary_default::_DEFAUL_DICTIONARY;
use crate::dictionary_stopword::_STOPWORD_DICTIONARY;

use std::collections::HashMap;

pub struct Dictionary {
    basic_words: HashMap<String, bool>,
}

impl Dictionary {
    // new generates dictionary for stemmer based on default words written on the code
    pub fn new() -> Dictionary {
        let mut mapped_words : HashMap<String, bool> = HashMap::new();
        generate_dictionary(&mut mapped_words, _DEFAUL_DICTIONARY);

        Dictionary{
            basic_words: mapped_words,
        }
    }

    // stopword generates dictionary for stop word remover based on default words for stopword written on the code
    pub fn stopword() -> Dictionary {
        let mut mapped_words : HashMap<String, bool> = HashMap::new();
        generate_dictionary(&mut mapped_words, _STOPWORD_DICTIONARY);

        Dictionary{
            basic_words: mapped_words,
        }
    }

    // find lets you search a word from dictionary safely
    pub fn find(&self, word: &str) -> bool {
        self.basic_words.get(word).unwrap().clone()
        // get() function to find the word inside the hashmap
        // unwrap() transforms form Option<&bool> to &bool
        // clone() takes the value from as the result, instead reference result
    }

    // TODO:
    // - add function for new word to dictionary
    // - remove function for remove word from dictionary
}

fn generate_dictionary(map: &mut HashMap<String,bool>, words: &[&str]) {
    for word in words.iter() {
        map.insert(word.to_string(), true);
    }
}