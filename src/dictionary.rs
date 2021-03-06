use crate::dictionary_default::_DEFAUL_DICTIONARY;
use crate::dictionary_stopword::_STOPWORD_DICTIONARY;

use std::collections::HashMap;

pub struct Dictionary {
    basic_words: HashMap<String, bool>,
}

impl Dictionary {
    // new generates dictionary for stemmer based on default words written on the code
    pub fn new() -> Dictionary {
        generate_dictionary(_DEFAUL_DICTIONARY)
    }

    // stopword generates dictionary for stop word remover based on default words for stopword written on the code
    pub fn stopword() -> Dictionary {
        generate_dictionary(_STOPWORD_DICTIONARY)
    }

    // if you have your own dataset, you can generate it here
    pub fn custom(words: &[&str]) -> Dictionary {
        generate_dictionary(words)
    }

    // find lets you search a word from dictionary safely
    pub fn find(&self, word: &String) -> bool {
        let found = self.basic_words.contains_key(word);
        if !found {
            return false
        }
        self.basic_words.get(word).unwrap().clone()
        // get() function to find the word inside the hashmap
        // unwrap() transforms form Option<&bool> to &bool
        // clone() takes the value from as the result, instead reference result
    }

    pub fn add<'a>(&'a mut self, word: &String) {
        self.basic_words.insert(word.to_string(), true);
    }

    // removing data is slow
    pub fn remove<'a>(&'a mut self, word: &String) {
        self.basic_words.remove(&word.to_string());
    }

    // since removing is slow, you have an option to disable the word
    pub fn disable(&self, word: &String) {
        self.basic_words.get(word).replace(&false);
    }

    // bring the word back to active state
    pub fn enable(&self, word: &String) {
        self.basic_words.get(word).replace(&true);
    }

    pub fn length(&self) -> usize {
        self.basic_words.len()
    }

    pub fn print(&self, separator: &String) {
        let mut separator = separator.clone();
        if separator == "" {
            separator = String::from(", ");
        }

        let mut index: usize = 0;
        let length: usize = self.basic_words.len();
        for (word, active) in self.basic_words.iter() {
            if *active {
                index += 1;
                if index >= length {
                    println!("{}", word);
                } else {
                    print!("{}{}", word, separator);
                }
            }
        }
    }
}

fn generate_dictionary(words: &[&str]) -> Dictionary {
    let mut map : HashMap<String, bool> = HashMap::new();

    for word in words.iter() {
        map.insert(word.to_string(), true);
    }

    Dictionary{
        basic_words: map,
    }
}