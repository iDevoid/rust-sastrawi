use crate::dictionary::Dictionary;
use regex::Regex;

pub struct Affixation<'a> {
    dictionary: &'a Dictionary,
    pub prefix_first: Regex,
    particle: Regex,
    possesive: Regex,
    suffix: Regex,
    regex_standard: Vec<standardRegex>,
}

struct standardRegex {
    regex_spec: Regex,
    returning_result: &'static [&'static str],
}

impl<'a> Affixation<'a> {
    pub fn new(dict: &Dictionary) -> Affixation {
        Affixation{
            dictionary: dict,
            prefix_first: Regex::new(r"^(be.+lah|be.+an|me.+i|di.+i|pe.+i|ter.+i)$").unwrap(),
            particle: Regex::new(r"-*(lah|kah|tah|pun)$").unwrap(),
            possesive: Regex::new(r"-*(ku|mu|nya)$").unwrap(),
            suffix: Regex::new(r"-*(is|isme|isasi|i|kan|an)$").unwrap(),
            regex_standard: vec![],
        }
    }

    pub fn remove_prefixes(&self, word: &String) -> (bool, String) {
        let original_word = word.clone();
        let mut current_prefix: String;
        let mut removed_prefix = String::from("");
        let mut recoding_chat: &[String];

        for _ in 1..4 {
            if word.len() < 3 {
                return (false, original_word)
            }

            current_prefix = word[..2].to_string();
            if current_prefix == removed_prefix {
                break
            }

            
        }

        (false, word.to_owned())
    }

    fn remove_prefix(&self, word: &String) -> (String, String, Vec<String>) {
        let mut recoding: Vec<String> = vec![];

        if word.starts_with("kau") {
            return ("kau".to_string(), word[3..].to_string(), recoding)
        }
        
        let mut prefix: String = word[..2].to_string();
        let mut result: String = String::from("");
        
        match word.as_str() {
            "di" | "ke" | "se" | "ku" => {
                result = word[2..].to_string();
            },
            _ => {
                
            }
        }

        (prefix, result, recoding)
    }
}

static _DEFAULT_PREFIX_ME: &[(&str, &[&str])] = &[
    (r"^me([lrwy][aiueo].*)$", &[])
];