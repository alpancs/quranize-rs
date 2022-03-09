#![feature(test)]

mod quran_index;
pub use quran_index::build_quran_index;

mod quranize_map;
pub use quranize_map::build_quranize_map;

mod quranize;
pub use quranize::build_quranize;
