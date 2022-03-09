#![feature(test)]

mod quran_index;
use quran_index::{build_quran_index, Harf};

mod quranize_map;
use quranize_map::{build_quranize_map, QuranizeMap};

pub struct Quranize {
    quran_index: Harf,
    quranize_map: QuranizeMap,
}

pub fn build_quranize() -> Quranize {
    Quranize {
        quran_index: build_quran_index(),
        quranize_map: build_quranize_map(),
    }
}

impl Quranize {
    pub fn encode(&self, alphabet: &str) -> String {
        self.encode_with_context(alphabet, &self.quran_index)
    }

    fn encode_with_context(&self, alphabet: &str, context: &Harf) -> String {
        String::new()
    }
}
