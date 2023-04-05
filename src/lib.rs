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

use std::{collections::HashMap, str::Chars};

mod collections;
use collections::Node;

mod normalization;
use normalization::{normalize, normalize_first_aya};

mod quran;
pub use quran::AyaGetter;
use quran::CleanCharsExt;

mod transliterations;
use transliterations as trans;

mod word_utils;
use word_utils::WordSuffixIterExt;

type HarfNode = Node<char>;
type EncodeResults<'a> = Vec<(String, Vec<&'a str>, usize)>;
type Location = (u8, u16, u8);

/// Struct to encode alphabetic text to quran text.
pub struct Quranize {
    root: HarfNode,
    locations_index: HashMap<*const HarfNode, Vec<Location>>,
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
        let mut quranize = Quranize {
            root: Default::default(),
            locations_index: Default::default(),
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
        let mut node = &mut self.root;
        let next_chars = quran.chars().skip(1).chain(std::iter::once(' '));
        for ((c, nc), n) in quran.chars().zip(next_chars).zip(1..) {
            node = node.get_mut_or_add(c);
            if nc == ' ' {
                self.locations_index.entry(node).or_default().push(location);
                if n >= min_harfs {
                    break;
                }
            }
        }
    }

    /// Encode `text` back into Quran form.
    pub fn encode(&self, text: &str) -> EncodeResults {
        let mut results = self.rev_encode(&self.root, &normalize(text));
        results.append(&mut self.rev_encode_first_aya(&self.root, &normalize_first_aya(text)));
        results.sort_unstable_by(|(q1, _, _), (q2, _, _)| q1.cmp(q2));
        results.dedup_by(|(q1, _, _), (q2, _, _)| q1 == q2);
        for (q, e, _) in results.iter_mut() {
            *q = q.chars().rev().collect();
            e.reverse();
        }
        results
    }

    fn rev_encode(&self, node: &HarfNode, text: &str) -> EncodeResults {
        let mut results = EncodeResults::new();
        if text.is_empty() {
            if let Some(locations) = self.locations_index.get(&(node as *const HarfNode)) {
                results.push((String::new(), Vec::new(), locations.len()));
            }
        }
        for n in node.iter() {
            let prefixes = trans::map(n.element)
                .iter()
                .chain(trans::contextual_map(node.element, n.element));
            for prefix in prefixes {
                if let Some(subtext) = text.strip_prefix(prefix) {
                    results.append(&mut self.rev_encode_sub(n, subtext, prefix));
                }
            }
        }
        results
    }

    fn rev_encode_sub<'a>(&'a self, n: &HarfNode, text: &str, expl: &'a str) -> EncodeResults {
        let mut results = self.rev_encode(n, text);
        for (q, e, _) in results.iter_mut() {
            q.push(n.element);
            e.push(expl);
        }
        results
    }

    fn rev_encode_first_aya(&self, node: &HarfNode, text: &str) -> EncodeResults {
        let mut results = EncodeResults::new();
        if text.is_empty() && self.containing_first_aya(node) {
            results.push((
                String::new(),
                Vec::new(),
                self.locations_index[&(node as *const HarfNode)].len(),
            ));
        }
        for n in node.iter() {
            for prefix in trans::single_harf_map(n.element) {
                if let Some(subtext) = text.strip_prefix(prefix) {
                    results.append(&mut self.rev_encode_sub_fa(n, subtext, prefix));
                }
            }
        }
        results
    }

    fn containing_first_aya(&self, node: &HarfNode) -> bool {
        self.locations_index
            .get(&(node as *const HarfNode))
            .map(|l| l.iter().any(|&(_, a, _)| a == 1))
            .unwrap_or_default()
    }

    fn rev_encode_sub_fa<'a>(&'a self, n: &HarfNode, text: &str, expl: &'a str) -> EncodeResults {
        let mut results = self.rev_encode_first_aya(n, text);
        for (q, e, _) in results.iter_mut() {
            q.push(n.element);
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
        self.get_locations_from(&self.root, quran.chars())
            .map(|v| v.as_slice())
            .unwrap_or_default()
    }

    fn get_locations_from(&self, node: &HarfNode, mut harfs: Chars) -> Option<&Vec<Location>> {
        match harfs.next() {
            Some(harf) => node
                .iter()
                .find(|n| n.element == harf)
                .and_then(|n| self.get_locations_from(n, harfs)),
            None => self.locations_index.get(&(node as *const HarfNode)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Quranize {
        fn e(&self, text: &str) -> Vec<String> {
            self.encode(text).into_iter().map(|(q, _, _)| q).collect()
        }
    }

    #[test]
    fn test_quranize_default() {
        let q = Quranize::default();

        assert_eq!(q.e("allah"), vec!["آلله", "الله"]);
        assert_eq!(q.e("illa billah"), vec!["إلا بالله"]);
        assert_eq!(q.e("alquran"), vec!["القرآن"]);
        assert_eq!(q.e("alqur'an"), vec!["القرآن"]);
        assert_eq!(q.e("bismillah"), vec!["بسم الله"]);
        assert_eq!(q.e("birobbinnas"), vec!["برب الناس"]);
        assert_eq!(q.e("inna anzalnahu"), vec!["إنا أنزلناه"]);
        assert_eq!(q.e("wa'tasimu"), vec!["واعتصموا"]);
        assert_eq!(q.e("wabarro"), vec!["وبرا", "وبئر"]);
        assert_eq!(q.e("idza qodho"), vec!["إذا قضى"]);
        assert_eq!(q.e("masyaallah"), vec!["ما شاء الله"]);
        assert_eq!(q.e("illa man taaba"), vec!["إلا من تاب"]);
        assert_eq!(q.e("qulhuwallahuahad"), vec!["قل هو الله أحد"]);
        assert_eq!(q.e("alla tahzani"), vec!["ألا تحزني"]);
        assert_eq!(q.e("innasya niaka"), vec!["إن شانئك"]);
        assert_eq!(q.e("wasalamun alaihi"), vec!["وسلام عليه"]);
        assert_eq!(q.e("ulaika hum"), vec!["أولئك هم"]);
        assert_eq!(q.e("waladdoolin"), vec!["ولا الضالين"]);
        assert_eq!(q.e("undur kaifa"), vec!["انظر كيف"]);
        assert_eq!(q.e("lirrohman"), vec!["للرحمن"]);

        assert_eq!(
            q.e("bismilla hirrohman nirrohiim"),
            vec!["بسم الله الرحمن الرحيم"]
        );
        assert_eq!(
            q.e("alhamdulilla hirobbil 'alamiin"),
            vec!["الحمد لله رب العالمين"]
        );
        assert_eq!(q.e("arrohma nirrohim"), vec!["الرحمن الرحيم"]);
        assert_eq!(q.e("maliki yau middin"), vec!["مالك يوم الدين"]);
        assert_eq!(
            q.e("iyyakanakbudu waiyyakanastain"),
            vec!["إياك نعبد وإياك نستعين"]
        );
        assert_eq!(
            q.e("ihdinassirotol mustaqim"),
            vec!["اهدنا الصراط المستقيم"]
        );
        assert_eq!(
            q.e("shirotolladzina an'amta 'alaihim ghoiril maghdzubi 'alaihim waladdoolliin"),
            vec!["صراط الذين أنعمت عليهم غير المغضوب عليهم ولا الضالين"]
        );
        assert_eq!(q.e("qulhuwallahuahad"), vec!["قل هو الله أحد"]);

        assert_eq!(q.e("alif lam mim"), vec!["الم"]);
        assert_eq!(q.e("alif laaam miiim"), vec!["الم"]);
        assert_eq!(q.e("nuun"), vec!["ن"]);
        assert_eq!(q.e("kaaaf haa yaa aiiin shoood"), vec!["كهيعص"]);
        assert_eq!(q.e("kaf ha ya 'ain shod"), vec!["كهيعص"]);

        assert_eq!(q.locations_index.len(), 685_770);
    }

    #[test]
    fn test_quranize_misc() {
        let q = Quranize::new(23);
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
        let q = Quranize::new(23);
        assert!(q.encode("").is_empty());
        assert!(q.encode("aaa").is_empty());
        assert!(q.encode("abcd").is_empty());
        assert!(q.encode("1+2=3").is_empty());
    }

    #[test]
    fn test_unique() {
        let q = Quranize::new(23);
        let texts = q.e("ALLAH");
        assert!(is_unique(texts.clone()), "{:#?}", texts);
    }

    fn is_unique(mut texts: Vec<String>) -> bool {
        texts.sort();
        texts
            .iter()
            .zip(texts.iter().skip(1))
            .fold(true, |acc, (t1, t2)| acc && t1 != t2)
    }

    #[test]
    fn test_locate() {
        let q = Quranize::new(23);
        assert_eq!(q.get_locations("بسم").first(), Some(&(1, 1, 1)));
        assert_eq!(q.get_locations("والناس").last(), Some(&(114, 6, 3)));
        assert_eq!(q.get_locations("بسم الله الرحمن الرحيم").len(), 2);
        assert_eq!(q.get_locations("ن").first(), Some(&(68, 1, 1)));
        assert!(q.get_locations("").is_empty());
        assert!(q.get_locations("نن").is_empty());
        assert!(q.get_locations("ننن").is_empty());
        assert!(q.get_locations("نننن").is_empty());
        assert!(q.get_locations("2+3+4=9").is_empty());
    }
}
