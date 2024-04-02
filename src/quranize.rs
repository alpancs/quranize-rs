use std::{collections::HashMap, iter::once, str::Chars};

mod collections;
use collections::Node;

mod normalization;
use normalization::{normalize, normalize_first_aya};

mod transliterations;
use transliterations::*;

mod word_utils;
use word_utils::WordSuffixIterExt;

type HarfNode = Node<char>;
type EncodeResult<'a> = (String, Vec<&'a str>, usize);
type EncodeResults<'a> = Vec<EncodeResult<'a>>;
type Location = (u8, u16, u8);
type Locations = Vec<Location>;

/// Struct to encode alphabetic text to quran text.
pub struct Quranize {
    root: HarfNode,
    locations_index: HashMap<*const HarfNode, Locations>,
}

impl Default for Quranize {
    /// Build [`Quranize`] with maximum `min_harfs` value.
    /// It is equivalent to building [`Quranize`] without any harf limits.
    ///
    /// # Examples
    ///
    /// ```
    /// let q = quranize::Quranize::default(); // the same with `Quranize::new(u16::MAX)`
    /// assert_eq!(q.encode("masyaallah").first().unwrap().0, "ما شاءَ اللَّهُ");
    /// ```
    fn default() -> Self {
        Self::new(u16::MAX)
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
    /// let q = quranize::Quranize::new(1);
    /// assert_eq!(q.encode("nun").first().unwrap().0, "ن");
    /// assert_eq!(q.encode("masyaallah").first(), None);
    /// ```
    pub fn new(min_harfs: u16) -> Self {
        let mut quranize = Quranize {
            root: Default::default(),
            locations_index: Default::default(),
        };
        for (s, a, q) in crate::quran::iter() {
            for (q, w) in clean_aya(q).word_suffixes().zip(1..) {
                quranize.index(q, (s, a, w), min_harfs);
            }
        }
        quranize
    }

    fn index(&mut self, quran: &str, location: Location, min_harfs: u16) {
        let mut node = &mut self.root;
        let next_chars = quran.chars().skip(1).chain(once(' '));
        for ((c, next_c), harfs) in quran.chars().zip(next_chars).zip(1..) {
            node = node.get_mut_or_add(c);
            if next_c == ' ' {
                self.locations_index.entry(node).or_default().push(location);
                if harfs >= min_harfs {
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
            let prefixes = map(n.element)
                .iter()
                .chain(contextual_map(node.element, n.element));
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
            for prefix in single_harf_map(n.element) {
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
    /// let q = quranize::Quranize::new(10);
    /// assert_eq!(q.get_locations("بِسمِ").first(), Some(&(1, 1, 1)));
    /// assert_eq!(q.get_locations("ن").first(), Some(&(68, 1, 1)));
    /// ```
    pub fn get_locations(&self, quran: &str) -> &[Location] {
        self.get_locations_from(&self.root, quran.chars())
            .map(|v| v.as_slice())
            .unwrap_or_default()
    }

    fn get_locations_from(&self, node: &HarfNode, mut harfs: Chars) -> Option<&Locations> {
        match harfs.next() {
            Some(harf) => node
                .iter()
                .find(|n| n.element == harf)
                .and_then(|n| self.get_locations_from(n, harfs)),
            None => self.locations_index.get(&(node as *const HarfNode)),
        }
    }
}

fn clean_aya(aya: &str) -> String {
    aya.chars().filter(|&c| mappable(c)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    impl Quranize {
        fn assert_encode(&self, text: &str, expected: &[&str]) {
            let actual: Vec<_> = self.encode(text).into_iter().map(|(q, _, _)| q).collect();
            assert_eq!(expected, actual, "text: {}", text);
        }
    }

    #[test]
    fn test_clean_aya() {
        for (s, a, q) in crate::quran::iter() {
            let q_words = q.word_suffixes().count();
            let clean_q_words = clean_aya(q).word_suffixes().count();
            assert_eq!(q_words, clean_q_words, "sura={} aya={}", s, a);
        }
    }

    #[test]
    fn test_quranize_default() {
        let q = Quranize::default();
        q.assert_encode("allah", &["اللَّهَ", "اللَّهُ", "ءاللَّهُ", "اللَّهِ"]);
        q.assert_encode("illa billah", &["إِلّا بِاللَّهِ"]);
        q.assert_encode("alquran", &["القُرءانَ", "القُرءانُ", "القُرءانِ"]);
        q.assert_encode("alqur'an", &["القُرءانَ", "القُرءانُ", "القُرءانِ"]);
        q.assert_encode("bismillah", &["بِسمِ اللَّهِ"]);
        q.assert_encode("birobbinnas", &["بِرَبِّ النّاسِ"]);
        q.assert_encode("inna anzalnahu", &["إِنّا أَنزَلنٰهُ"]);
        q.assert_encode("wa'tasimu", &["وَاعتَصِموا"]);
        q.assert_encode("wa'tasimu bihablillah", &["وَاعتَصِموا بِحَبلِ اللَّهِ"]);
        q.assert_encode("wabarro", &["وَبَرًّا"]);
        q.assert_encode("idza qodho", &["إِذا قَضَى"]);
        q.assert_encode("idza qodhoo", &["إِذا قَضَى", "إِذا قَضىٰ"]);
        q.assert_encode("masyaallah", &["ما شاءَ اللَّهُ"]);
        q.assert_encode("illa man taba", &["إِلّا مَن تابَ"]);
        q.assert_encode("qulhuwallahuahad", &["قُل هُوَ اللَّهُ أَحَدٌ"]);
        q.assert_encode("alla tahzani", &["أَلّا تَحزَنى"]);
        q.assert_encode("innasya niaka", &["إِنَّ شانِئَكَ"]);
        q.assert_encode("innasya ni'aka", &["إِنَّ شانِئَكَ"]);
        q.assert_encode("wasalamun alaihi", &["وَسَلٰمٌ عَلَيهِ"]);
        q.assert_encode("ulaika hum", &["أُولٰئِكَ هُم", "أُولٰئِكَ هُمُ"]);
        q.assert_encode("waladdoollin", &["وَلَا الضّالّينَ"]);
        q.assert_encode("undur kaifa", &["انظُر كَيفَ"]);
        q.assert_encode("lirrohman", &["لِلرَّحمٰنِ"]);
        q.assert_encode("wantum muslimun", &["وَأَنتُم مُسلِمونَ"]);
        q.assert_encode("laa yukallifullah", &["لا يُكَلِّفُ اللَّهُ"]);
        q.assert_encode("robbil alamin", &["رَبِّ العٰلَمينَ"]);
        q.assert_encode("husnul maab", &["حُسنُ المَـٔابِ"]);
        q.assert_encode("kufuwan", &["كُفُوًا"]);
        q.assert_encode("yukhodiun", &["يُخٰدِعونَ"]);
        q.assert_encode("indallah", &["عِندَ اللَّهِ", "عِندِ اللَّهِ"]);
        q.assert_encode("alimul ghoibi", &["عٰلِمُ الغَيبِ"]);
        q.assert_encode("kaana dhoifa", &["كانَ ضَعيفًا"]);
        q.assert_encode("kitabi la roiba", &["الكِتٰبِ لا رَيبَ"]);
        q.assert_encode("takwili", &["تَأويلِ"]);
        q.assert_encode("yu'minun", &["يُؤمِنونَ"]);
        q.assert_encode("hudan lil muttaqin", &["هُدًى لِلمُتَّقينَ"]);
    }

    #[test]
    fn test_first_aya() {
        let q = Quranize::new(25);
        q.assert_encode("alif lam mim", &["الم"]);
        q.assert_encode("alif laaam miiim", &["الم"]);
        q.assert_encode("nuun", &["ن"]);
        q.assert_encode("kaaaf haa yaa aiiin shoood", &["كهيعص"]);
        q.assert_encode("kaf ha ya 'ain shod", &["كهيعص"]);
    }

    #[test]
    fn test_alfatihah() {
        let q = Quranize::new(100);
        q.assert_encode("bismillahirrohmanirrohiim", &["بِسمِ اللَّهِ الرَّحمٰنِ الرَّحيمِ"]);
        q.assert_encode("alhamdulilla hirobbil 'alamiin", &["الحَمدُ لِلَّهِ رَبِّ العٰلَمينَ"]);
        q.assert_encode("arrohma nirrohim", &["الرَّحمٰنِ الرَّحيمِ"]);
        q.assert_encode("maliki yau middin", &["مٰلِكِ يَومِ الدّينِ"]);
        q.assert_encode("iyyakanakbudu waiyyakanastain", &["إِيّاكَ نَعبُدُ وَإِيّاكَ نَستَعينُ"]);
        q.assert_encode("ihdinassirotol mustaqim", &["اهدِنَا الصِّرٰطَ المُستَقيمَ"]);
        q.assert_encode(
            "shirotolladzina an'amta 'alaihim ghoiril maghdzubi 'alaihim waladdoolliin",
            &["صِرٰطَ الَّذينَ أَنعَمتَ عَلَيهِم غَيرِ المَغضوبِ عَلَيهِم وَلَا الضّالّينَ"],
        );
    }

    #[test]
    fn test_al_ikhlas() {
        let q = Quranize::new(50);
        q.assert_encode("qulhuwallahuahad", &["قُل هُوَ اللَّهُ أَحَدٌ"]);
        q.assert_encode("allahussomad", &["اللَّهُ الصَّمَدُ"]);
        q.assert_encode("lam yalid walam yulad", &["لَم يَلِد وَلَم يولَد"]);
        q.assert_encode("walam yakun lahu kufuwan ahad", &["وَلَم يَكُن لَهُ كُفُوًا أَحَدٌ"]);
    }

    #[test]
    fn test_quranize_misc() {
        let q = Quranize::new(70);
        assert!(matches!(q.encode("bismillah").first(), Some((_, _, 3))));
        assert_eq!(
            q.encode("alhamdu").into_iter().next().unwrap().1,
            vec!["a", "l", "h", "a", "m", "d", "u"]
        );
        assert_eq!(
            q.encode("arrohman").into_iter().next().unwrap().1,
            vec!["a", "", "r", "r", "o", "h", "m", "a", "n", ""]
        );
        {
            let r = q.encode("masyaallah").into_iter().next().unwrap();
            assert_eq!(r.0.chars().count(), r.1.len());
        }
        {
            let r = q.encode("birobbinnas").into_iter().next().unwrap();
            assert_eq!(
                r.1.into_iter().zip(r.0.chars()).collect::<Vec<_>>(),
                vec![
                    ("b", 'ب',),
                    ("i", '\u{650}',),
                    ("r", 'ر',),
                    ("o", '\u{64e}',),
                    ("b", 'ب',),
                    ("b", '\u{651}',),
                    ("i", '\u{650}',),
                    ("", ' ',),
                    ("", 'ا',),
                    ("", 'ل',),
                    ("n", 'ن',),
                    ("n", '\u{651}',),
                    ("a", 'ا',),
                    ("s", 'س',),
                    ("", '\u{650}',),
                ]
            );
        }
    }

    #[test]
    fn test_quranize_empty_result() {
        let q = Quranize::new(23);
        assert!(q.encode("").is_empty());
        assert!(q.encode("abcd").is_empty());
        assert!(q.encode("1+2=3").is_empty());
    }

    #[test]
    fn test_unique() {
        let q = Quranize::new(23);
        let results = q.encode("ALLAH");
        let qurans = results.iter().map(|(q, _, _)| q);
        let unique_count = std::collections::HashSet::<&String>::from_iter(qurans).len();
        assert_eq!(results.len(), unique_count, "{:#?}", results);
    }

    #[test]
    fn test_locate() {
        let q = Quranize::new(70);
        assert_eq!(q.get_locations("بِسمِ").first(), Some(&(1, 1, 1)));
        assert_eq!(q.get_locations("وَالنّاسِ").last(), Some(&(114, 6, 3)));
        assert_eq!(q.get_locations("بِسمِ اللَّهِ الرَّحمٰنِ الرَّحيمِ").len(), 2);
        assert_eq!(q.get_locations("ن").first(), Some(&(68, 1, 1)));
        assert!(q.get_locations("").is_empty());
        assert!(q.get_locations("نن").is_empty());
        assert!(q.get_locations("ننن").is_empty());
        assert!(q.get_locations("نننن").is_empty());
        assert!(q.get_locations("2+3+4=9").is_empty());
    }
}
