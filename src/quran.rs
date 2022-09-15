//! Helper module related to Quran stuffs.

use std::{iter::Filter, str::Chars};

mod simple_plain;
use simple_plain::RAW_QURAN as SIMPLE_PLAIN;

const SURA_COUNT: usize = 114;
const AYA_COUNT: usize = 6236;

/// Accept raw Quran string, return an iterator for each ayah in the Quran with surah number and ayah number.
pub fn iter() -> impl Iterator<Item = (u8, u16, &'static str)> {
    iter_quran(SIMPLE_PLAIN)
}

fn iter_quran(raw: &str) -> impl Iterator<Item = (u8, u16, &str)> {
    let raw = raw.trim_start();
    let basmalah = raw.split('\n').next().unwrap().split('|').nth(2).unwrap();
    let basmalah_prefix = basmalah.to_string() + " ";
    raw.split('\n').take(AYA_COUNT).map(move |l| {
        let mut it = l.split('|');
        let sura_number = it.next().unwrap().parse().unwrap();
        let aya_number = it.next().unwrap().parse().unwrap();
        let mut aya_text = it.next().unwrap();
        if sura_number != 1 && sura_number != 9 && aya_number == 1 {
            aya_text = aya_text.strip_prefix(&basmalah_prefix).unwrap();
        }
        (sura_number, aya_number, aya_text)
    })
}

pub trait CleanCharsExt {
    fn clean_chars(&self) -> Filter<Chars, fn(&char) -> bool>;
}

use crate::transliterations as trans;
impl CleanCharsExt for str {
    fn clean_chars(&self) -> Filter<Chars, fn(&char) -> bool> {
        self.chars().filter(|&c| !trans::map(c).is_empty())
    }
}

/// Struct to get ayah texts by surah number and ayah number.
pub struct AyaGetter<'a> {
    aya_texts: Vec<&'a str>,
    aya_sums: Vec<usize>,
}

impl Default for AyaGetter<'_> {
    fn default() -> Self {
        AyaGetter::new()
    }
}
impl<'a> AyaGetter<'a> {
    /// Create a new `AyaGetter`.
    fn new() -> Self {
        let mut aya_texts = Vec::with_capacity(AYA_COUNT);
        let mut aya_sums = Vec::with_capacity(SURA_COUNT);
        for (i, (_, a, q)) in iter_quran(SIMPLE_PLAIN).enumerate() {
            aya_texts.push(q);
            if a == 1 {
                aya_sums.push(i);
            }
        }
        Self {
            aya_texts,
            aya_sums,
        }
    }

    /// Get an ayah text given a surah number and an ayah number.
    ///
    /// # Examples
    ///
    /// ```
    /// use quranize::AyaGetter;
    /// let aya_getter = AyaGetter::default();
    /// assert_eq!(aya_getter.get(1, 1), Some("بِسْمِ اللَّهِ الرَّحْمَـٰنِ الرَّحِيمِ"));
    /// assert_eq!(aya_getter.get(114, 6), Some("مِنَ الْجِنَّةِ وَالنَّاسِ"));
    /// ```
    pub fn get(&self, sura_number: u8, aya_number: u16) -> Option<&'a str> {
        let aya_sum = self.aya_sums.get(sura_number as usize - 1)?;
        Some(*self.aya_texts.get(aya_sum + aya_number as usize - 1)?)
    }
}

#[cfg(test)]
mod simple_clean;

#[cfg(test)]
use simple_clean::RAW_QURAN as SIMPLE_CLEAN;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quran_version_compatibility() {
        let word_counter =
            |(s, a, t): (u8, u16, &'static str)| (s, a, t, t.split_whitespace().count());
        let simple_clean_wc_iter = iter_quran(SIMPLE_CLEAN).map(word_counter);
        let simple_plain_wc_iter = iter_quran(SIMPLE_PLAIN).map(word_counter);
        for ((s1, a1, t1, c1), (_, _, t2, c2)) in simple_clean_wc_iter.zip(simple_plain_wc_iter) {
            assert_eq!(c1, c2, "sura_number = {s1}, aya_number = {a1},\naya_text = {t1} and {t2},\nword count = {c1} and {c2}");
        }
    }

    #[test]
    fn test_properties() {
        assert_same_basmalah(SIMPLE_CLEAN);
        assert_eq!(iter_quran(SIMPLE_CLEAN).count(), AYA_COUNT);
        {
            assert_same_basmalah(SIMPLE_PLAIN);
            assert_eq!(iter_quran(SIMPLE_PLAIN).count(), AYA_COUNT);
        }
        assert_eq!(count_unique_simple_clean_chars(), 37);
        assert_eq!(count_unique_simple_plain_chars(), 37);
    }

    fn assert_same_basmalah(raw: &str) {
        let mut lines = raw.trim_start().split('\n').take(AYA_COUNT);
        let basmalah = lines.next().unwrap().split('|').nth(2).unwrap();
        let basmalah = basmalah.to_string() + " ";
        for line in lines {
            let mut parts = line.split('|');
            let sura_number: u8 = parts.next().unwrap().parse().unwrap();
            let aya_number: u16 = parts.next().unwrap().parse().unwrap();
            let aya_text = parts.next().unwrap();
            if aya_number == 1 && sura_number != 9 {
                assert!(
                    aya_text.starts_with(&basmalah),
                    "sura_number = {sura_number}, aya_number = {aya_number},\naya_text = {aya_text}"
                );
            }
        }
    }

    fn count_unique_simple_clean_chars() -> usize {
        let mut set = std::collections::HashSet::new();
        for (_, _, t) in iter_quran(SIMPLE_CLEAN) {
            set.extend(t.chars());
        }
        set.len()
    }

    fn count_unique_simple_plain_chars() -> usize {
        let mut set = std::collections::HashSet::new();
        for (_, _, t) in iter() {
            set.extend(t.clean_chars());
        }
        set.len()
    }

    #[test]
    fn test_map() {
        let aya_getter = AyaGetter::new();
        assert_eq!(aya_getter.get(1, 1), Some("بِسْمِ اللَّهِ الرَّحْمَـٰنِ الرَّحِيمِ"));
        assert_eq!(aya_getter.get(114, 6), Some("مِنَ الْجِنَّةِ وَالنَّاسِ"));
        assert_eq!(aya_getter.get(114, 7), None);
    }

    #[test]
    fn test_clean_chars() {
        for ((_, _, clean), (_, _, plain)) in iter_quran(SIMPLE_CLEAN).zip(iter()) {
            assert_eq!(clean, plain.clean_chars().collect::<String>());
        }
    }
}
