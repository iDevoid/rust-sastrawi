use crate::dictionary::Dictionary;
use crate::tokenizer::Tokenizer;

pub struct Stemmer {
    dictionary: Dictionary,
    stopword: Dictionary,
    tokenizer: Tokenizer,
}

impl Stemmer {
    pub fn new() -> Stemmer {
        let dictionary = Dictionary::new();
        let stopword = Dictionary::stopword();
        let tokenizer = Tokenizer::new();
        Stemmer{
            dictionary: dictionary,
            stopword: stopword,
            tokenizer: tokenizer
        }
    }

    pub fn stem(&self, sentence: &String) -> Vec<String> {
        let words = self.tokenizer.tokenize(sentence);
        let mut results : Vec<String> = Vec::new();
        
        for word in words.iter() {
            if word.chars().count() < 3 {
                results.push(word.clone());
            }

            if self.dictionary.find(word) {
                results.push(word.clone());
            }
        }

        //TODO : 
        // - check for prefix, particle, suffix, possesive, etc

        results
    }

    pub fn stop_word(&self, sentence: &mut String) {
        let words = self.tokenizer.tokenize(sentence);
        let mut results: Vec<String> = Vec::new();

        for word in words.iter() {
            if !self.stopword.find(word) {
                results.push(word.clone());
            }
        }

        *sentence = results.join(" ");
    }
}