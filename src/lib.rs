#![feature(test)]

mod quran_index;

mod quranize_map;

pub struct Quranize {
    _quran_index: quran_index::Harf,
    _quranize_map: quranize_map::QuranizeMap,
}

pub fn build_quranize() -> Quranize {
    Quranize {
        _quran_index: quran_index::build_quran_index(),
        _quranize_map: quranize_map::build_quranize_map(),
    }
}

impl Quranize {
    pub fn encode(_: &str) -> String {
        String::new()
    }
}
