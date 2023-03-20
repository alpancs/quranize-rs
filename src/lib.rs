//! Encodes alphabetic text to quran text.
//! See [`Quranize`] for details.
//!
//! # Examples
//!
//! ## Adding crate quranize to a project's dependencies
//!
//! ```toml
//! [dependencies]
//! quranize = "0.10"
//! ```
//!
//! ## Encoding alphabetic text to quran text
//!
//! ```
//! let q = quranize::Quranize::default();
//! assert_eq!(q.encode("alhamdulillah").first().unwrap().0, "الحمد لله");
//! ```
//!
//! ## Getting an aya text given surah number and ayah number
//!
//! ```
//! let aya_getter = quranize::AyaGetter::default();
//! assert_eq!(aya_getter.get(1, 1), Some("بِسْمِ اللَّهِ الرَّحْمَـٰنِ الرَّحِيمِ"));
//! ```

mod normalization;
mod quran;
mod transliterations;
mod word_utils;

use std::{collections::HashMap, str::Chars};

use normalization::{normalize, normalize_first_aya};
pub use quran::AyaGetter;
use quran::CleanCharsExt;
use transliterations as trans;
use word_utils::WordSuffixIterExt;

type EncodeResults<'a> = Vec<(String, Vec<&'a str>, usize)>;
type Location = (u8, u16, u8);
type NodeIndex = usize;

/// Struct to encode alphabetic text to quran text.
pub struct Quranize {
    adjacencies: Vec<Vec<NodeIndex>>,
    harfs: Vec<char>,
    locs_map: HashMap<NodeIndex, Vec<Location>>,
    node_id: NodeIndex,
}

impl Default for Quranize {
    /// Build [`Quranize`] with maximum `min_harfs` value.
    /// It is equivalent to building [`Quranize`] without any harf limits.
    ///
    /// # Examples
    ///
    /// ```
    /// let q = quranize::Quranize::default(); // the same with `Quranize::new(usize::MAX)`
    /// assert_eq!(q.encode("masyaallah").first().unwrap().0, "ما شاء الله");
    /// ```
    fn default() -> Self {
        Self::new(usize::MAX)
    }
}

impl Quranize {
    /// Build [`Quranize`] with parameter `min_harfs`.
    /// The indexer will only scan quran harfs at least as many as `min_harfs` and stop at the nearest end of words.
    /// This strategy is implemented to reduce memory usage and indexing time.
    /// Use [`Quranize::default`] to build [`Quranize`] with maximum `min_harfs` value (without limits).
    ///
    /// # Examples
    ///
    /// ```
    /// let q = quranize::Quranize::new(35);
    /// assert_eq!(q.encode("masyaallah").first().unwrap().0, "ما شاء الله");
    /// let q = quranize::Quranize::new(1);
    /// assert_eq!(q.encode("masyaallah").first(), None);
    /// ```
    pub fn new(min_harfs: usize) -> Self {
        let mut quranize = Self {
            adjacencies: vec![vec![]],
            harfs: vec![0 as char],
            locs_map: Default::default(),
            node_id: 0,
        };
        for (s, a, q) in quran::iter() {
            let q = q.clean_chars().collect::<String>();
            for (q, w) in q.word_suffixes().zip(1..) {
                quranize.expand(q, (s, a, w), min_harfs);
            }
        }
        quranize
    }

    fn expand(&mut self, quran: &str, location: Location, min_harfs: usize) {
        let mut i = 0;
        let next_chars = quran.chars().skip(1).chain(std::iter::once(' '));
        for ((c, nc), n) in quran.chars().zip(next_chars).zip(1..) {
            i = self.get_or_add(i, c);
            if nc == ' ' {
                self.locs_map.entry(i).or_default().push(location);
                if n >= min_harfs {
                    break;
                }
            }
        }
    }

    fn get_or_add(&mut self, i: NodeIndex, harf: char) -> NodeIndex {
        match self.adjacencies[i].iter().find(|&&j| self.harfs[j] == harf) {
            Some(&j) => j,
            None => {
                self.node_id += 1;
                self.adjacencies.push(vec![]);
                self.harfs.push(harf);
                self.adjacencies[i].push(self.node_id);
                self.node_id
            }
        }
    }

    /// Encode `text` back into Quran form.
    pub fn encode(&self, text: &str) -> EncodeResults {
        let mut results = self.rev_encode(0, &normalize(text));
        results.append(&mut self.rev_encode_first_aya(0, &normalize_first_aya(text)));
        results.sort();
        results.dedup_by(|(q1, _, _), (q2, _, _)| q1 == q2);
        for (q, e, _) in results.iter_mut() {
            *q = q.chars().rev().collect();
            e.reverse();
        }
        results
    }

    fn rev_encode(&self, i: NodeIndex, text: &str) -> EncodeResults {
        let mut results = EncodeResults::new();
        if text.is_empty() {
            if let Some(locs) = self.locs_map.get(&i) {
                results.push((String::new(), Vec::new(), locs.len()));
            }
        }
        for &j in &self.adjacencies[i] {
            let prefixes = trans::map(self.harfs[j])
                .iter()
                .chain(trans::contextual_map(self.harfs[i], self.harfs[j]));
            for prefix in prefixes {
                if let Some(subtext) = text.strip_prefix(prefix) {
                    results.append(&mut self.rev_encode_sub(j, subtext, prefix));
                }
            }
        }
        results
    }

    fn rev_encode_sub<'a>(&'a self, i: NodeIndex, text: &str, expl: &'a str) -> EncodeResults {
        let mut results = self.rev_encode(i, text);
        for (q, e, _) in results.iter_mut() {
            q.push(self.harfs[i]);
            e.push(expl);
        }
        results
    }

    fn rev_encode_first_aya(&self, i: NodeIndex, text: &str) -> EncodeResults {
        let mut results = EncodeResults::new();
        if text.is_empty() && self.containing_first_aya(i) {
            results.push((String::new(), Vec::new(), self.locs_map[&i].len()));
        }
        for &j in &self.adjacencies[i] {
            for prefix in trans::single_harf_map(self.harfs[j]) {
                if let Some(subtext) = text.strip_prefix(prefix) {
                    results.append(&mut self.rev_encode_sub_fa(j, subtext, prefix));
                }
            }
        }
        results
    }

    fn containing_first_aya(&self, i: NodeIndex) -> bool {
        self.locs_map
            .get(&i)
            .map(|l| l.iter().any(|&(_, a, _)| a == 1))
            .unwrap_or_default()
    }

    fn rev_encode_sub_fa<'a>(&'a self, i: NodeIndex, text: &str, expl: &'a str) -> EncodeResults {
        let mut results = self.rev_encode_first_aya(i, text);
        for (q, e, _) in results.iter_mut() {
            q.push(self.harfs[i]);
            e.push(expl);
        }
        results
    }

    /// Get locations from the given `quran` text.
    /// Each location is a reference to a tuple that contains "sura number", "aya number", and "word number" within the aya.
    ///
    /// # Examples
    ///
    /// ```
    /// let q = quranize::Quranize::new(5);
    /// assert_eq!(q.get_locations("بسم").first(), Some(&(1, 1, 1)));
    /// assert_eq!(q.get_locations("ن").first(), Some(&(68, 1, 1)));
    /// ```
    pub fn get_locations(&self, quran: &str) -> &[Location] {
        self.get_locations_from(0, quran.chars())
            .map(|v| v.as_slice())
            .unwrap_or_default()
    }

    fn get_locations_from(&self, i: NodeIndex, mut harfs: Chars) -> Option<&Vec<Location>> {
        match harfs.next() {
            Some(harf) => self.adjacencies[i]
                .iter()
                .find(|&&j| self.harfs[j] == harf)
                .and_then(|&j| self.get_locations_from(j, harfs)),
            None => self.locs_map.get(&i),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Quranize {
        fn quran_results(&self, text: &str) -> Vec<String> {
            self.encode(text).into_iter().map(|(q, _, _)| q).collect()
        }
    }

    #[test]
    fn test_build_root() {
        let q = Quranize::new(1);
        assert_eq!(q.harfs[0], '\0');
        assert_eq!(q.adjacencies[0].len(), 31);
    }

    #[test]
    fn test_quranize_short() {
        let q = Quranize::new(21);
        assert_eq!(q.quran_results("allah"), vec!["آلله", "الله"]);
        assert_eq!(q.quran_results("illa billah"), vec!["إلا بالله"]);
        assert_eq!(q.quran_results("alquran"), vec!["القرآن"]);
        assert_eq!(q.quran_results("alqur'an"), vec!["القرآن"]);
        assert_eq!(q.quran_results("bismillah"), vec!["بسم الله"]);
        assert_eq!(q.quran_results("birobbinnas"), vec!["برب الناس"]);
        assert_eq!(q.quran_results("inna anzalnahu"), vec!["إنا أنزلناه"]);
        assert_eq!(q.quran_results("wa'tasimu"), vec!["واعتصموا"]);
        assert_eq!(q.quran_results("wabarro"), vec!["وبرا", "وبئر"]);
        assert_eq!(q.quran_results("idza qodho"), vec!["إذا قضى"]);
        assert_eq!(q.quran_results("masyaallah"), vec!["ما شاء الله"]);
        assert_eq!(q.quran_results("illa man taaba"), vec!["إلا من تاب"]);
        assert_eq!(q.quran_results("qulhuwallahuahad"), vec!["قل هو الله أحد"]);
        assert_eq!(q.quran_results("alla tahzani"), vec!["ألا تحزني"]);
        assert_eq!(q.quran_results("innasya niaka"), vec!["إن شانئك"]);
        assert_eq!(q.quran_results("wasalamun alaihi"), vec!["وسلام عليه"]);
        assert_eq!(q.quran_results("ulaika hum"), vec!["أولئك هم"]);
        assert_eq!(q.quran_results("waladdoolin"), vec!["ولا الضالين"]);
        assert_eq!(q.quran_results("undur kaifa"), vec!["انظر كيف"]);
        assert_eq!(q.quran_results("lirrohman"), vec!["للرحمن"]);
    }

    #[test]
    fn test_first_aya() {
        let q = Quranize::new(1);
        assert_eq!(q.quran_results("alif lam mim"), vec!["الم"]);
        assert_eq!(q.quran_results("alif laaam miiim"), vec!["الم"]);
        assert_eq!(q.quran_results("nuun"), vec!["ن"]);
        assert_eq!(q.quran_results("kaaaf haa yaa aiiin shoood"), vec!["كهيعص"]);
        assert_eq!(q.quran_results("kaf ha ya 'ain shod"), vec!["كهيعص"]);
    }

    #[test]
    fn test_quranize_full() {
        let q = Quranize::default();

        assert_eq!(q.adjacencies.len(), 3_483_437);
        let leaves_count = q.adjacencies.iter().filter(|v| v.is_empty()).count();
        assert_eq!(leaves_count, 66_697);

        assert_eq!(
            q.quran_results("bismilla hirrohman nirrohiim"),
            vec!["بسم الله الرحمن الرحيم"]
        );
        assert_eq!(
            q.quran_results("alhamdulilla hirobbil 'alamiin"),
            vec!["الحمد لله رب العالمين"]
        );
        assert_eq!(q.quran_results("arrohma nirrohim"), vec!["الرحمن الرحيم"]);
        assert_eq!(q.quran_results("maliki yau middin"), vec!["مالك يوم الدين"]);
        assert_eq!(
            q.quran_results("iyyakanakbudu waiyyakanastain"),
            vec!["إياك نعبد وإياك نستعين"]
        );
        assert_eq!(
            q.quran_results("ihdinassirotol mustaqim"),
            vec!["اهدنا الصراط المستقيم"]
        );
        assert_eq!(
            q.quran_results(
                "shirotolladzina an'amta 'alaihim ghoiril maghdzubi 'alaihim waladdoolliin"
            ),
            vec!["صراط الذين أنعمت عليهم غير المغضوب عليهم ولا الضالين"]
        );
        assert_eq!(q.quran_results("qulhuwallahuahad"), vec!["قل هو الله أحد"]);
    }

    #[test]
    fn test_quranize_misc() {
        let q = Quranize::new(14);
        assert_eq!(q.encode("bismillah")[0].1.len(), 8);
        assert_eq!(q.encode("bismillah")[0].2, 3);
        assert_eq!(q.encode("arrohman").len(), 1);
        assert_eq!(q.encode("arrohman")[0].1.len(), 6);
        assert_eq!(q.encode("alhamdu")[0].1, vec!["al", "ha", "m", "du"]);
        assert_eq!(
            q.encode("arrohman")[0].1,
            vec!["a", "", "ro", "h", "ma", "n"]
        );
        let result = &q.encode("masyaallah")[0];
        assert_eq!(result.0.chars().count(), result.1.len());
        assert_eq!(
            result.1,
            vec!["m", "a", "", "sy", "a", "a", "", "", "l", "la", "h"]
        );
    }

    #[test]
    fn test_quranize_empty_result() {
        let q = Quranize::new(14);
        assert!(q.encode("").is_empty());
        assert!(q.encode("aaa").is_empty());
        assert!(q.encode("abcd").is_empty());
        assert!(q.encode("1+2=3").is_empty());
    }

    #[test]
    fn test_unique() {
        let q = Quranize::new(14);
        let texts = q.quran_results("ALLAH");
        assert!(is_unique(&texts), "{:#?}", texts);
    }

    fn is_unique(texts: &[String]) -> bool {
        let mut texts = texts.to_owned();
        texts.sort();
        texts
            .iter()
            .zip(texts.iter().skip(1))
            .fold(true, |acc, (t1, t2)| acc && t1 != t2)
    }

    #[test]
    fn test_locate() {
        let q = Quranize::new(21);
        assert_eq!(q.get_locations("بسم").first(), Some(&(1, 1, 1)));
        assert_eq!(q.get_locations("والناس").last(), Some(&(114, 6, 3)));
        assert_eq!(q.get_locations("بسم الله الرحمن الرحيم").len(), 2);
        assert_eq!(q.get_locations("").first(), None);
        assert_eq!(q.get_locations("ن").first(), Some(&(68, 1, 1)));
        assert_eq!(q.get_locations("نن").first(), None);
        assert_eq!(q.get_locations("ننن").first(), None);
        assert_eq!(q.get_locations("نننن").first(), None);
        assert_eq!(q.get_locations("2+3+4=9"), []);
    }
}
