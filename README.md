# rust-sastrawi
rust-sastrawi is a Rust Library based from [PHP Sastrawi](https://github.com/sastrawi/sastrawi) made by [Andy Librian](https://github.com/andylibrian), which allows you to do Stemming and StopWord Removal in Bahasa Indonesia (Indonesian Language)

rust-sastrawi adalah library untuk Rust Language yang didasarkan dari [PHP Sastrawi](https://github.com/sastrawi/sastrawi) yang dibuat oleh [Andy Librian](https://github.com/andylibrian), digunakan untuk Stemming dan StopWord Removal pada Bahasa Indonesia

## Penggunaan
1. Menambahkan Dependency di Cargo.tompl

```toml
.
[dependencies]
sastrawi = "0.1.0"
```
2. mengunduh Dependency dengan terminal anda dengan syntax

`cargo check`


3. contoh penggunaan sastrawi
```rust
use sastrawi::*;

fn main() {
    let dict = Dictionary::new();
    let stemmer = Stemmer::new(&dict);
    let sentence = String::from("Perekonomian Indonesia sedang dalam pertumbuhan yang membanggakan");
    let stemmed_words = stemmer.stem_sentence(&sentence);
    for word in stemmed_words.iter() {
        println!("{}", word);
    }
}
```


## Pustaka

#### Algoritma

1. Algoritma Nazief dan Adriani
2. Asian J. 2007. ___Effective Techniques for Indonesian Text Retrieval___. PhD thesis School of Computer Science and Information Technology RMIT University Australia. ([PDF](http://researchbank.rmit.edu.au/eserv/rmit:6312/Asian.pdf) dan [Amazon](https://www.amazon.com/Effective-Techniques-Indonesian-Text-Retrieval/dp/3639021649))
3. Arifin, A.Z., I.P.A.K. Mahendra dan H.T. Ciptaningtyas. 2009. ___Enhanced Confix Stripping Stemmer and Ants Algorithm for Classifying News Document in Indonesian Language___, Proceeding of International Conference on Information & Communication Technology and Systems (ICTS). ([PDF](http://personal.its.ac.id/files/pub/2623-agusza-baru%2021%20d%20VIP%20enhanced-confix-stripping-stem.pdf))
4. A. D. Tahitoe, D. Purwitasari. 2010. ___Implementasi Modifikasi Enhanced Confix Stripping Stemmer Untuk Bahasa Indonesia dengan Metode Corpus Based Stemming___, Institut Teknologi Sepuluh Nopember (ITS) – Surabaya, 60111, Indonesia. ([PDF](http://digilib.its.ac.id/public/ITS-Undergraduate-14255-paperpdf.pdf))
5. Tambahan aturan _stemming_ dari [kontributor Sastrawi](https://github.com/sastrawi/sastrawi/graphs/contributors).

#### Kamus Kata Dasar

Proses stemming oleh Sastrawi sangat bergantung pada kamus kata dasar. Sastrawi menggunakan kamus kata dasar dari [kateglo.com](http://kateglo.com) dengan sedikit perubahan.

## Lisensi

Sebagaimana [Sastrawi](https://github.com/sastrawi/sastrawi) untuk PHP, Go-Sastrawi juga disebarkan dengan lisensi [MIT](http://choosealicense.com/licenses/mit/). Untuk lisensi kamus kata dasar dari Kateglo adalah [CC-BY-NC-SA 3.0](https://github.com/ivanlanin/kateglo#lisensi-isi).

## Di Bahasa Pemrograman Lain

- [Sastrawi](https://github.com/sastrawi/sastrawi) - PHP
- [JSastrawi](https://github.com/jsastrawi/jsastrawi) - Java
- [cSastrawi](https://github.com/mohangk/c_sastrawi) - C
- [PySastrawi](https://github.com/har07/PySastrawi) - Python
- [Sastrawi-Ruby](https://github.com/meisyal/sastrawi-ruby) - Ruby
- [Go-Sastrawi](https://github.com/RadhiFadlillah/go-sastrawi) - Golang