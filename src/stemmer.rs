use crate::dictionary::Dictionary;
use crate::tokenizer::Tokenizer;
use crate::affixation::Affixation;

pub struct Stemmer<'a> {
    dictionary: &'a Dictionary,
    tokenizer: Tokenizer,
    affixation: Affixation<'a>,
}

impl<'a> Stemmer<'a> {
    pub fn new(dictionary: &Dictionary) -> Stemmer {
        let tokenizer = Tokenizer::new();
        let affixation = Affixation::new(dictionary);
        Stemmer{
            dictionary: dictionary,
            tokenizer: tokenizer,
            affixation: affixation,
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

            //TODO : 
            // - check for prefix, particle, suffix, possesive, etc
            if self.affixation.prefix_first.is_match(word) {

            } else {
                
            }
        }

        results
    }
}