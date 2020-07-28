extern crate regex;

pub fn assign_regex_prefix_pe() -> [(regex::Regex, Vec<String>); 15] {
    [(
        // Pattern 1 : pe{w|y}V => pe-{w|y}V
        regex::Regex::new(r"^pe([wy][aiueo].*)$").unwrap(), vec![]
    ),
    (
        // Pattern 2 : perV => per-V OR pe-rV
        regex::Regex::new(r"^per([aiueo].*)$").unwrap(), vec![String::from("r")]
    ),
    (
        // Pattern 3 : perCAP => per-CAP where C != 'r' and P != 'er'
        regex::Regex::new(r"^per([^aiueor][a-z][^e].*)$").unwrap(), vec![]
    ),
    (
        // Pattern 4 : perCAerV => per-CAerV where C != 'r'
        regex::Regex::new(r"^per([^aiueor][a-z]er[aiueo].*)$").unwrap(), vec![]
    ),
    (
        // Pattern 5 : pem{b|f|v} => pem-{b|f|v}
        regex::Regex::new(r"^pem([bfv].*)$").unwrap(), vec![]
    ),
    (
        // Pattern 6 : pem{rV|V} => pe-m{rV|V} OR pe-p{rV|V}
        regex::Regex::new(r"^pem(r?[aiueo].*)$").unwrap(), vec![String::from("m"), String::from("p")]
    ),
    (
        // Pattern 7 : pen{c|d|j|s|t|z} => pen-{c|d|j|s|t|z}
        regex::Regex::new(r"^pen([cdjstz].*)$").unwrap(), vec![]
    ),
    (
        // Pattern 8 : penV => pe-nV OR pe-tV
        regex::Regex::new(r"^pen([aiueo].*)$").unwrap(), vec![String::from("n"), String::from("t")]
    ),
    (
        // Pattern 9 : pengC => peng-C
        regex::Regex::new(r"^peng([^aiueo].*)$").unwrap(), vec![]
    ),
    (
        // Pattern 10 : pengV => peng-V OR peng-kV OR pengV- where V = 'e'
        regex::Regex::new(r"^peng(([aiueo])(.*))$").unwrap(), vec![String::from("k")]
    ),
    (
        // Pattern 11 : penyV => peny-sV OR pe-nyV
        regex::Regex::new(r"^peny([aiueo].*)$").unwrap(), vec![String::from("s"), String::from("ny")]
    ),
    (
        // Pattern 12 : pelV => pe-lV OR pel-V for pelajar
        regex::Regex::new(r"^pe(l[aiueo].*)$").unwrap(), vec![]
    ),
    (
        // Pattern 13 : peCerV => peC-erV where C != {r|w|y|l|m|n}
        regex::Regex::new(r"^pe[^aiueorwylmn](er[aiueo].*)$").unwrap(), vec![]
    ),
    (
        // Pattern 14 : peCP => pe-CP where C != {r|w|y|l|m|n} and P != 'er'
        regex::Regex::new(r"^pe([^aiueorwylmn][^e].*)$").unwrap(), vec![]
    ),
    (
        // Pattern 15 : peC1erC2 => pe-C1erC2 where C1 != {r|w|y|l|m|n}
        regex::Regex::new(r"^pe([^aiueorwylmn]er[^aiueo].*)$").unwrap(), vec![]
    )]
}

pub fn assign_regex_prefix_be() -> [(regex::Regex, Vec<String>); 5] {
    [(
        // Pattern 1 : berV => ber-V OR be-rV           the recoding returns r value
        regex::Regex::new(r"ber([aiueo].*)$").unwrap(), vec![String::from("r")]
    ),
    (
        // Pattern 2 : berCAP => ber-CAP where C != 'r' and P != 'er'
        regex::Regex::new(r"^ber([^aiueor][a-z][^e].*)$").unwrap(), vec![]
    ),
    (
        // Pattern 3 : berCAerV => ber-CAerV where C != 'r'
        regex::Regex::new(r"^ber([^aiueor][a-z]er[aiueo].*)$").unwrap(), vec![]
    ),
    (
        // Pattern 4 : belajar => bel-ajar
        regex::Regex::new(r"^bel(ajar)$").unwrap(), vec![]
    ),
    (
        // Pattern 5 : beC1erC2 => be-C1erC2 where C1 != {'r'|'l'}
        regex::Regex::new(r"^be([^aiueorl]er[^aiueo].*)$").unwrap(), vec![]
    )]
}

pub fn assign_regex_prefix_te() -> [(regex::Regex, Vec<String>); 5] {
    [(
        // Pattern 1 : terV => ter-V OR te-rV           the recoding returns r value
        regex::Regex::new(r"^ter([aiueo].*)$").unwrap(), vec![String::from("r")]
    ),
    (
        // Pattern 2 : terCerV => ter-CerV where C != 'r'
        regex::Regex::new(r"^ter([^aiueor]er[aiueo].*)$").unwrap(), vec![]
    ),
    (
        // Pattern 3 : terCP => ter-CP where C != 'r' and P != 'er'
        regex::Regex::new(r"^ter([^aiueor][^e].*)$").unwrap(), vec![]
    ),
    (
        // Pattern 4 : teC1erC2 => te-C1erC2 where C1 != 'r'
        regex::Regex::new(r"^te([^aiueor]er[^aiueo].*)$").unwrap(), vec![]
    ),
    (
        // Pattern 5 : terC1erC2 => ter-C1erC2 where C1 != 'r'
        regex::Regex::new(r"^ter([^aiueor]er[^aiueo].*)$").unwrap(), vec![]
    )]
}

pub fn assign_regex_infix() -> [regex::Regex; 2] {
    [
        regex::Regex::new(r"^(([^aiueo])e[rlm])([aiueo].*)$").unwrap(), // Pattern 1 : CerV => CerV OR CV
        regex::Regex::new(r"^(([^aiueo])in)([aiueo].*)$").unwrap(), // Pattern 2 : CinV => CinV OR CV
    ]
}
