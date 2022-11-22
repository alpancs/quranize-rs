//! Encodes alphabetic text to quran text.
//! See [`Quranize`] for details.
//!
//! # Examples
//!
//! ## Adding crate quranize to a project's dependencies
//!
//! ```toml
//! [dependencies]
//! quranize = "0.7"
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
mod quran_index;
mod transliterations;
mod word_utils;

pub use quran::AyaGetter;
use quran_index::{EncodeResults, Location, Node};
use word_utils::WordSuffixIterExt;

/// Struct to encode alphabetic text to quran text.
pub struct Quranize {
    root: Node,
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
        let mut root = Node::new('\0');
        for (s, a, q) in quran::iter() {
            for (i, q) in q.word_suffixes().enumerate() {
                root.expand(q, (s, a, i as u8 + 1), min_harfs);
            }
        }
        Self { root }
    }

    /// Encode `text` back into Quran form.
    pub fn encode(&self, text: &str) -> EncodeResults {
        let mut results = self.root.rev_encode(&normalization::normalize(text));
        results.sort();
        results.dedup_by(|(q1, _), (q2, _)| q1 == q2);
        for (q, e) in results.iter_mut() {
            *q = q.chars().rev().collect();
            e.reverse();
        }
        results
    }

    /// Get locations from the given `quran` text.
    /// Each location is a reference to a tuple that contains sura number, aya number, and word number within the aya.
    ///
    /// Note that the locations are returned in descending order (from the last word of الناس to the first word of الفاتحة).
    ///
    /// # Examples
    ///
    /// ```
    /// let q = quranize::Quranize::new(5);
    /// assert_eq!(q.get_locations("بسم").last(), Some(&(1, 1, 1)));
    /// assert_eq!(q.get_locations("ن").next(), Some(&(68, 1, 1)));
    /// ```
    pub fn get_locations(&self, quran: &str) -> impl Iterator<Item = &Location> {
        match self.root.get_locations(quran) {
            None => self.root.locations.iter(),
            Some(locations) => locations.iter(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_root() {
        let root = Quranize::new(1).root;
        assert_eq!(root.content, '\0');
        assert_eq!(root.next_harfs.len(), 31);
        assert_eq!(root.get('ب').unwrap().locations.len(), 0);
        assert_eq!(root.get('ن').unwrap().locations.len(), 1);
    }

    #[test]
    fn test_quranize_short() {
        let q = Quranize::new(21);
        assert_eq!(encode(&q, "allah"), vec!["آلله", "الله"]);
        assert_eq!(encode(&q, "alquran"), vec!["القرآن"]);
        assert_eq!(encode(&q, "alqur'an"), vec!["القرآن"]);
        assert_eq!(encode(&q, "bismillah"), vec!["بسم الله"]);
        assert_eq!(encode(&q, "birobbinnas"), vec!["برب الناس"]);
        assert_eq!(encode(&q, "inna anzalnahu"), vec!["إنا أنزلناه"]);
        assert_eq!(encode(&q, "wa'tasimu"), vec!["واعتصموا"]);
        assert_eq!(encode(&q, "wabarro"), vec!["وبرا", "وبئر"]);
        assert_eq!(encode(&q, "idza qodho"), vec!["إذا قضى"]);
        assert_eq!(encode(&q, "masyaallah"), vec!["ما شاء الله"]);
        assert_eq!(encode(&q, "illa man taaba"), vec!["إلا من تاب"]);
        assert_eq!(encode(&q, "qulhuwallahuahad"), vec!["قل هو الله أحد"]);
        assert_eq!(encode(&q, "alla tahzani"), vec!["ألا تحزني"]);
        assert_eq!(encode(&q, "innasya niaka"), vec!["إن شانئك"]);
        assert_eq!(encode(&q, "wasalamun alaihi"), vec!["وسلام عليه"]);
        assert_eq!(encode(&q, "ulaika hum"), vec!["أولئك هم"]);
        assert_eq!(encode(&q, "waladdoolin"), vec!["ولا الضالين"]);
        assert_eq!(encode(&q, "n"), vec!["ن"]);
        assert_eq!(encode(&q, "undur kaifa"), vec!["انظر كيف"]);
        assert_eq!(encode(&q, "lirrohman"), vec!["للرحمن"]);
    }

    #[test]
    fn test_quranize_full() {
        let q = Quranize::default();
        assert_eq!(
            encode(&q, "bismilla hirrohman nirrohiim"),
            vec!["بسم الله الرحمن الرحيم"]
        );
        assert_eq!(
            encode(&q, "alhamdulilla hirobbil 'alamiin"),
            vec!["الحمد لله رب العالمين"]
        );
        assert_eq!(encode(&q, "arrohma nirrohim"), vec!["الرحمن الرحيم"]);
        assert_eq!(encode(&q, "maliki yau middin"), vec!["مالك يوم الدين"]);
        assert_eq!(
            encode(&q, "iyyakanakbudu waiyyakanastain"),
            vec!["إياك نعبد وإياك نستعين"]
        );
        assert_eq!(
            encode(&q, "ihdinassirotol mustaqim"),
            vec!["اهدنا الصراط المستقيم"]
        );
        assert_eq!(
            encode(
                &q,
                "shirotolladzina an'amta 'alaihim ghoiril maghdzubi 'alaihim waladdoolliin"
            ),
            vec!["صراط الذين أنعمت عليهم غير المغضوب عليهم ولا الضالين"]
        );
        assert_eq!(encode(&q, "qulhuwallahuahad"), vec!["قل هو الله أحد"]);
    }

    fn encode(quranize: &Quranize, text: &str) -> Vec<String> {
        quranize.encode(text).into_iter().map(|(q, _)| q).collect()
    }

    #[test]
    fn test_quranize_misc() {
        let q = Quranize::new(21);
        assert_eq!(q.encode("bismillah")[0].1.len(), 8);
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
        assert!(q.encode("bbb").is_empty());
        assert!(q.encode("abcd").is_empty());
        assert!(q.encode("1+2=3").is_empty());
    }

    #[test]
    fn test_unique() {
        let q = Quranize::new(14);
        let texts = encode(&q, "allah");
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
        let q = Quranize::new(35);
        assert_eq!(q.get_locations("بسم").last(), Some(&(1, 1, 1)));
        assert_eq!(q.get_locations("والناس").next(), Some(&(114, 6, 3)));
        assert_eq!(q.get_locations("بسم الله الرحمن الرحيم").count(), 2);
        assert_eq!(q.get_locations("").next(), None);
        assert_eq!(q.get_locations("ن").next(), Some(&(68, 1, 1)));
        assert_eq!(q.get_locations("نن").next(), None);
        assert_eq!(q.get_locations("ننن").next(), None);
        assert_eq!(q.get_locations("نننن").next(), None);
        assert_eq!(q.get_locations("2+3+4=9").next(), None);
    }
}
