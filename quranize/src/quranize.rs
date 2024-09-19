mod suffix_tree;
use suffix_tree::{Data as Loc, Edge, SuffixTree};

mod normalization;
use normalization::normalize;

mod transliterations;
use transliterations::*;

use crate::quran;

mod word_suffixes;

type EncodeResults = Vec<(String, Vec<&'static str>)>;

/// Struct to encode alphabetic text to quran text.
pub struct Quranize {
    st: SuffixTree<'static>,
    pub saq_pairs: Vec<(u8, u16, &'static str)>,
}

impl Default for Quranize {
    fn default() -> Self {
        Self::new()
    }
}

impl Quranize {
    /// Create a new [`Quranize`] instance, including building the suffix tree internally.
    ///
    /// # Examples
    ///
    /// ```
    /// let q = quranize::Quranize::new();
    /// assert_eq!("ما شاءَ اللَّه", q.encode("masyaallah").first().unwrap().0);
    /// ```
    pub fn new() -> Self {
        let mut st = SuffixTree::new();
        let mut saq_pairs = Vec::with_capacity(quran::AYA_COUNT);
        for (i, (s, a, q)) in quran::iter().enumerate() {
            st.construct(i, q);
            saq_pairs.push((s, a, q.trim()));
        }
        Self { st, saq_pairs }
    }

    fn labeled_edged(&self, v: usize) -> impl Iterator<Item = &Edge> {
        self.st.v_edges(v).filter(|(_, _, l)| !l.is_empty())
    }

    /// Encode `text` into Quran form.
    pub fn encode(&self, text: &str) -> EncodeResults {
        let s = &normalize(text);
        let mut results: Vec<_> = self
            .labeled_edged(0)
            .flat_map(|&e| self.rev_encode(s, e, None))
            .collect();
        // results.append(&mut self.rev_encode_first_aya(&self.root, &normalize_first_aya(text)));
        results.sort_unstable_by(|(q1, _), (q2, _)| q1.cmp(q2));
        results.dedup_by(|(q1, _), (q2, _)| q1 == q2);
        for (q, e) in results.iter_mut() {
            *q = q.chars().rev().collect();
            e.reverse();
        }
        results
    }

    fn rev_encode(&self, s: &str, e: Edge, m: Option<(char, &'static str)>) -> EncodeResults {
        match (s, e.2.chars().next()) {
            ("", _) => m
                .into_iter()
                .map(|(c, p)| (c.to_string(), vec![p]))
                .collect(),
            (_, Some(c)) => { map(c).iter().chain(contextual_map(m.unzip().0, c)) }
                .filter_map(|&p| Some(p).zip(s.strip_prefix(p)))
                .flat_map(|(p, s)| {
                    match &e.2[c.len_utf8()..] {
                        "" => self
                            .labeled_edged(e.1)
                            .flat_map(|&e| self.rev_encode(s, e, Some((c, p))))
                            .collect(),
                        l => self.rev_encode(s, (e.0, e.1, l), Some((c, p))),
                    }
                    .into_iter()
                    .map(|mut subresult| {
                        if let Some((c, p)) = m {
                            subresult.0.push(c);
                            subresult.1.push(p);
                        }
                        subresult
                    })
                })
                .collect(),
            _ => vec![],
        }
    }

    /// Find locations from the given `quran` text.
    /// Each location is a reference to a tuple that contains "row offset", and "column offset".
    ///
    /// # Examples
    ///
    /// ```
    /// let q = quranize::Quranize::new();
    /// assert_eq!(Some(&(0, 0)), q.find_str("بِسمِ").first());
    /// ```
    pub fn find_str(&self, s: &str) -> Vec<Loc> {
        self.st.find_str(s, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::collections::HashSet;

    impl Quranize {
        fn assert_encode(&self, text: &str, expected: &[&str]) {
            let (actual, _): (Vec<_>, Vec<_>) = self.encode(text).into_iter().unzip();
            assert_eq!(expected, actual, "text: {}", text);
        }
    }

    #[test]
    fn test_node_count() {
        let q = Quranize::new();
        assert_eq!(q.st.vertex_count(), 116_077);
    }

    #[test]
    fn test_quranize_default() {
        let q = Quranize::new();
        q.assert_encode("illa billah", &["إِلّا بِاللَّه"]);
        q.assert_encode("alqur'an", &["القُرءان"]);
        q.assert_encode("bismillah", &["بِسمِ اللَّه"]);
        q.assert_encode("birobbinnas", &["بِرَبِّ النّاس"]);
        q.assert_encode("inna anzalnahu", &["إِنّا أَنزَلنٰهُ"]);
        q.assert_encode("wa'tasimu", &["وَاعتَصِمو"]);
        q.assert_encode("wa'tasimu bihablillah", &["وَاعتَصِموا بِحَبلِ اللَّه"]);
        q.assert_encode("idza qodho", &["إِذا قَضَ"]);
        q.assert_encode("masyaallah", &["ما شاءَ اللَّه"]);
        q.assert_encode("illa man taba", &["إِلّا مَن تابَ"]);
        q.assert_encode("alla tahzani", &["أَلّا تَحزَنى"]);
        q.assert_encode("innasya niaka", &["إِنَّ شانِئَكَ"]);
        q.assert_encode("innasya ni'aka", &["إِنَّ شانِئَكَ"]);
        q.assert_encode("wasalamun alaihi", &["وَسَلٰمٌ عَلَيهِ"]);
        q.assert_encode("ulaika hum", &["أُولٰئِكَ هُم"]);
        q.assert_encode("waladdoollin", &["وَلَا الضّالّين"]);
        q.assert_encode("undur kaifa", &["انظُر كَيفَ"]);
        q.assert_encode("lirrohman", &["لِلرَّحمٰن"]);
        q.assert_encode("waantum muslimun", &["وَأَنتُم مُسلِمون"]);
        q.assert_encode("laa yukallifullah", &["لا يُكَلِّفُ اللَّه"]);
        q.assert_encode("robbil alamin", &["رَبِّ العٰلَمين"]);
        q.assert_encode("husnul maab", &["حُسنُ المَـٔاب"]);
        q.assert_encode("khusnul ma'ab", &["حُسنُ المَـٔاب"]);
        q.assert_encode("kufuwan", &["كُفُوً"]);
        q.assert_encode("yukhodiun", &["يُخٰدِعون"]);
        q.assert_encode("indallah", &["عِندَ اللَّه"]);
        q.assert_encode("alimul ghoibi", &["عٰلِمُ الغَيبِ"]);
        q.assert_encode("kaana dhoifa", &["كانَ ضَعيفًا"]);
        q.assert_encode("waantum muslimuna", &["وَأَنتُم مُسلِمونَ"]);
        q.assert_encode("kitabi la roiba", &["الكِتٰبِ لا رَيبَ"]);
        q.assert_encode("takwili", &["تَأويلِ"]);
        q.assert_encode("yu'minun", &["يُؤمِنون"]);
        q.assert_encode("hudan lil muttaqin", &["هُدًى لِلمُتَّقين"]);
        q.assert_encode("majreeha wamursaha", &["مَجر۪ىٰها وَمُرسىٰها"]);
    }

    // #[test]
    // fn test_first_aya() {
    //     let q = Quranize::new(25);
    //     q.assert_encode("alif lam mim", &["الم"]);
    //     q.assert_encode("alif laaam miiim", &["الم"]);
    //     q.assert_encode("nuun", &["ن"]);
    //     q.assert_encode("kaaaf haa yaa aiiin shoood", &["كهيعص"]);
    //     q.assert_encode("kaf ha ya 'ain shod", &["كهيعص"]);
    // }

    #[test]
    fn test_alfatihah() {
        let q = Quranize::new();
        q.assert_encode("bismillahirrohmanirrohiim", &["بِسمِ اللَّهِ الرَّحمٰنِ الرَّحيم"]);
        q.assert_encode("alhamdulilla hirobbil 'alamiin", &["الحَمدُ لِلَّهِ رَبِّ العٰلَمين"]);
        q.assert_encode("arrohma nirrohim", &["الرَّحمٰنِ الرَّحيم"]);
        q.assert_encode("maliki yau middin", &["مٰلِكِ يَومِ الدّين"]);
        q.assert_encode("iyyakanakbudu waiyyakanastain", &["إِيّاكَ نَعبُدُ وَإِيّاكَ نَستَعين"]);
        q.assert_encode("ihdinassirotol mustaqim", &["اهدِنَا الصِّرٰطَ المُستَقيم"]);
        q.assert_encode(
            "shirotolladzina an'amta 'alaihim ghoiril maghdzubi 'alaihim waladdoolliin",
            &["صِرٰطَ الَّذينَ أَنعَمتَ عَلَيهِم غَيرِ المَغضوبِ عَلَيهِم وَلَا الضّالّين"],
        );
    }

    #[test]
    fn test_al_ikhlas() {
        let q = Quranize::new();
        q.assert_encode("qulhuwallahuahad", &["قُل هُوَ اللَّهُ أَحَد"]);
        q.assert_encode("allahussomad", &["اللَّهُ الصَّمَد"]);
        q.assert_encode("lam yalid walam yulad", &["لَم يَلِد وَلَم يولَد"]);
        q.assert_encode("walam yakun lahu kufuwan ahad", &["وَلَم يَكُن لَهُ كُفُوًا أَحَد"]);
    }

    #[test]
    fn test_quranize_misc() {
        let q = Quranize::new();
        assert_eq!(
            vec!["a", "l", "h", "a", "m", "d", "u"],
            q.encode("alhamdu").first().unwrap().1,
        );
        assert_eq!(
            vec!["a", "", "r", "r", "o", "h", "m", "a", "n"],
            q.encode("arrohman").first().unwrap().1,
        );
        {
            let (q, e) = q.encode("masyaallah").into_iter().next().unwrap();
            assert!(q.chars().count() == e.len());
        }
        {
            let (q, e) = q.encode("birobbinnas").into_iter().next().unwrap();
            assert_eq!(
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
                ],
                e.into_iter().zip(q.chars()).collect::<Vec<_>>(),
            );
        }
    }

    #[test]
    fn test_quranize_empty_result() {
        let q = Quranize::new();
        assert!(q.encode("").is_empty(), "result={:?}", q.encode(""));
        assert!(q.encode("abcd").is_empty());
        assert!(q.encode("1+2=3").is_empty());
    }

    #[test]
    fn test_unique() {
        let q = Quranize::new();
        let results = q.encode("ALLAH");
        let qurans = results.iter().map(|(q, _)| q);
        let is_unique = results.len() == HashSet::<&String>::from_iter(qurans).len();
        assert!(is_unique, "results are not unique. results: {:#?}", results);
    }

    #[test]
    fn test_find_str() {
        let q = Quranize::new();

        assert_eq!(q.find_str("بِسمِ").first(), Some(&(0, 0)));
        assert_eq!(q.find_str("وَالنّاسِ").last(), Some(&(6235, 28)));
        assert_eq!(q.find_str("الم").first(), Some(&(7, 0)));
        assert_eq!(q.find_str("بِسمِ اللَّهِ الرَّحمٰنِ الرَّحيمِ").len(), 2);
        assert!(q.find_str("").is_empty());
        assert!(q.find_str("نن").is_empty());
        assert!(q.find_str("ننن").is_empty());
        assert!(q.find_str("نننن").is_empty());
        assert!(q.find_str("2+3+4=9").is_empty());

        assert_eq!(q.find_str("بِسمِ اللَّهِ الرَّحمٰنِ الرَّحيم").first(), Some(&(0, 0)));
        assert_eq!(q.find_str("الرَّحمٰنِ الرَّحيم").first(), Some(&(0, 26)));
        assert_eq!(q.find_str("").first(), None);
        assert_eq!(q.find_str("abc").first(), None);
    }
}
