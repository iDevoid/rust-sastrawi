use regex::Regex;
use snailquote::unescape;
use htmlescape::decode_html;

pub struct Tokenizer {
    properties : Vec<RegexProperties>,
}

struct RegexProperties {
    str_regex : Regex,
    str_replacement : &'static str,
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        let mut regexes : Vec<RegexProperties> = Vec::new();
        for reg_format in _DEFAULT_REGEX_FORMAT.iter() {
            regexes.push(RegexProperties{
                str_regex: Regex::new(reg_format.0).unwrap(),
                str_replacement: reg_format.1,
            });
        }
        Tokenizer{
            properties: regexes,
        }
    }

    pub fn tokenize(&self, sentence: &str) -> Vec<String> {
        let mut _cleaned_sentence : String;
        _cleaned_sentence = sentence.to_lowercase();
        _cleaned_sentence = unescape(sentence).unwrap();
        _cleaned_sentence = decode_html(&_cleaned_sentence).unwrap();        
    
        for regex_property in &self.properties {
            _cleaned_sentence = regex_property.str_regex.replace_all(&_cleaned_sentence, regex_property.str_replacement).to_string();
        }

        _cleaned_sentence = _cleaned_sentence.trim().to_string();
                
        vec![_cleaned_sentence.split_whitespace().collect()]
    }
}

// don't change the order, this follows the structure of actual algorithm
static _DEFAULT_REGEX_FORMAT: &[(&str, &str)] = &[
    // URL
    (r"(?i)(www\.|https?|s?ftp)\S+", ""),
    
    // email
    (r"(?i)\S+@\S+", ""),

    // twitter
    (r"(?i)(@|#)\S+", ""),

    // escape string
    (r"(?i)&.*;", ""),

    // symbols
    (r"(?i)[^a-z\s]", "")
];