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
type EncodeResults<'a> = Vec<(String, Vec<&'a str>, usize)>;
type Location = (u8, u16, u8);

/// Struct to encode alphabetic text to quran text.
pub struct Quranize {
    root: HarfNode,
    node_locations: HashMap<*const HarfNode, Vec<Location>>,
}

impl Default for Quranize {
    /// Build [`Quranize`] with maximum `min_harfs` value.
    /// It is equivalent to building [`Quranize`] without any harf limits.
    ///
    /// # Examples
    ///
    /// ```
    /// let q = quranize::Quranize::default(); // the same with `Quranize::new(usize::MAX)`
    /// assert_eq!(q.encode("masyaallah").first().unwrap().0, "ما شاءَ اللَّهُ");
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
    /// let q = quranize::Quranize::new(1);
    /// assert_eq!(q.encode("nun").first().unwrap().0, "ن");
    /// assert_eq!(q.encode("masyaallah").first(), None);
    /// ```
    pub fn new(min_harfs: usize) -> Self {
        let mut quranize = Quranize {
            root: Default::default(),
            node_locations: Default::default(),
        };
        for (s, a, q) in crate::quran::iter() {
            for (q, w) in clean_aya(q).word_suffixes().zip(1..) {
                quranize.index(q, (s, a, w), min_harfs);
            }
        }
        quranize
    }

    fn index(&mut self, quran: &str, location: Location, min_harfs: usize) {
        let mut node = &mut self.root;
        let next_chars = quran.chars().skip(1).chain(once(' '));
        for ((c, next_c), harfs) in quran.chars().zip(next_chars).zip(1..) {
            node = node.get_mut_or_add(c);
            if next_c == ' ' {
                self.node_locations.entry(node).or_default().push(location);
                if harfs >= min_harfs {
                    break;
                }
            }
        }
    }

    /// Encode `text` back into Quran form.
    pub fn encode(&self, text: &str) -> EncodeResults {
        let mut results = self.rev_encode(&self.root, &normalize(text), &mut String::new());
        results.append(&mut self.rev_encode_first_aya(&self.root, &normalize_first_aya(text)));
        results.sort_unstable_by(|(q1, _, _), (q2, _, _)| q1.cmp(q2));
        results.dedup_by(|(q1, _, _), (q2, _, _)| q1 == q2);
        for (q, e, _) in results.iter_mut() {
            *q = q.chars().rev().collect();
            e.reverse();
        }
        results
    }

    fn rev_encode(&self, node: &HarfNode, text: &str, ctx: &mut String) -> EncodeResults {
        let mut results = EncodeResults::new();
        if text.is_empty() {
            if let Some(locations) = self.node_locations.get(&(node as *const HarfNode)) {
                results.push((String::new(), Vec::new(), locations.len()));
            }
        }
        ctx.push(node.element);
        for subnode in node.iter() {
            let prefixes = { map1(subnode.element).iter() }
                .chain(map2(node.element, subnode.element))
                .chain(mapn(ctx, subnode.element));
            for prefix in prefixes {
                if let Some(subtext) = text.strip_prefix(prefix) {
                    let mut subresults = self.rev_encode(subnode, subtext, ctx);
                    for (q, e, _) in subresults.iter_mut() {
                        q.push(subnode.element);
                        e.push(prefix);
                    }
                    results.append(&mut subresults);
                }
            }
        }
        ctx.pop();
        results
    }

    fn rev_encode_first_aya(&self, node: &HarfNode, text: &str) -> EncodeResults {
        let mut results = EncodeResults::new();
        if text.is_empty() && self.containing_first_aya(node) {
            results.push((
                String::new(),
                Vec::new(),
                self.node_locations[&(node as *const HarfNode)].len(),
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
        self.node_locations
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

    fn get_locations_from(&self, node: &HarfNode, mut harfs: Chars) -> Option<&Vec<Location>> {
        match harfs.next() {
            Some(harf) => node
                .iter()
                .find(|n| n.element == harf)
                .and_then(|n| self.get_locations_from(n, harfs)),
            None => self.node_locations.get(&(node as *const HarfNode)),
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
        fn e(&self, text: &str) -> Vec<String> {
            self.encode(text).into_iter().map(|(q, _, _)| q).collect()
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
        assert_eq!(q.e("allah"), vec!["اللَّهَ", "اللَّهُ", "ءاللَّهُ", "اللَّهِ"]);
        assert_eq!(q.e("illa billah"), vec!["إِلّا بِاللَّهِ"]);
        assert_eq!(q.e("alquran"), vec!["القُرءانَ", "القُرءانُ", "القُرءانِ"]);
        assert_eq!(q.e("alqur'an"), vec!["القُرءانَ", "القُرءانُ", "القُرءانِ"]);
        assert_eq!(q.e("bismillah"), vec!["بِسمِ اللَّهِ"]);
        assert_eq!(q.e("birobbinnas"), vec!["بِرَبِّ النّاسِ"]);
        assert_eq!(q.e("inna anzalnahu"), vec!["إِنّا أَنزَلنٰهُ"]);
        assert_eq!(q.e("wa'tasimu"), vec!["وَاعتَصِموا"]);
        assert_eq!(q.e("wa'tasimu bihablillah"), vec!["وَاعتَصِموا بِحَبلِ اللَّهِ"]);
        assert_eq!(q.e("wabarro"), vec!["وَبَرًّا"]);
        assert_eq!(q.e("idza qodho"), vec!["إِذا قَضَى"]);
        assert_eq!(q.e("idza qodhoo"), vec!["إِذا قَضَى", "إِذا قَضىٰ"]);
        assert_eq!(q.e("masyaallah"), vec!["ما شاءَ اللَّهُ"]);
        assert_eq!(q.e("illa man taba"), vec!["إِلّا مَن تابَ"]);
        assert_eq!(q.e("qulhuwallahuahad"), vec!["قُل هُوَ اللَّهُ أَحَدٌ"]);
        assert_eq!(q.e("alla tahzani"), vec!["أَلّا تَحزَنى"]);
        assert_eq!(q.e("innasya niaka"), vec!["إِنَّ شانِئَكَ"]);
        assert_eq!(q.e("innasya ni'aka"), vec!["إِنَّ شانِئَكَ"]);
        assert_eq!(q.e("wasalamun alaihi"), vec!["وَسَلٰمٌ عَلَيهِ"]);
        assert_eq!(q.e("ulaika hum"), vec!["أُولٰئِكَ هُم", "أُولٰئِكَ هُمُ"]);
        assert_eq!(q.e("waladdoollin"), vec!["وَلَا الضّالّينَ"]);
        assert_eq!(q.e("undur kaifa"), vec!["انظُر كَيفَ"]);
        assert_eq!(q.e("lirrohman"), vec!["لِلرَّحمٰنِ"]);
        assert_eq!(q.e("wantum muslimun"), vec!["وَأَنتُم مُسلِمونَ"]);
        assert_eq!(q.e("laa yukallifullah"), vec!["لا يُكَلِّفُ اللَّهُ"]);
        assert_eq!(q.e("robbil alamin"), vec!["رَبِّ العٰلَمينَ"]);
        assert_eq!(q.e("husnul maab"), vec!["حُسنُ المَـٔابِ"]);
        assert_eq!(q.e("kufuwan"), vec!["كُفُوًا"]);
        assert_eq!(q.e("yukhodiun"), vec!["يُخٰدِعونَ"]);
        assert_eq!(q.e("indallah"), vec!["عِندَ اللَّهِ", "عِندِ اللَّهِ"]);
        assert_eq!(q.e("alimul ghoibi"), vec!["عٰلِمُ الغَيبِ"]);
        assert_eq!(q.e("kaana dhoifa"), vec!["كانَ ضَعيفًا"]);
        assert_eq!(q.e("kitabi la roiba"), vec!["الكِتٰبِ لا رَيبَ"]);
        assert_eq!(q.e("takwili"), vec!["تَأويلِ"]);
        assert_eq!(q.e("yu'minun"), vec!["يُؤمِنونَ"]);
        assert_eq!(q.e("hudan lil muttaqin"), vec!["هُدًى لِلمُتَّقينَ"]);
    }

    #[test]
    fn test_first_aya() {
        let q = Quranize::new(25);
        assert_eq!(q.e("alif lam mim"), vec!["الم"]);
        assert_eq!(q.e("alif laaam miiim"), vec!["الم"]);
        assert_eq!(q.e("nuun"), vec!["ن"]);
        assert_eq!(q.e("kaaaf haa yaa aiiin shoood"), vec!["كهيعص"]);
        assert_eq!(q.e("kaf ha ya 'ain shod"), vec!["كهيعص"]);
    }

    #[test]
    fn test_alfatihah() {
        let q = Quranize::new(100);
        assert_eq!(
            q.e("bismillahirrohmanirrohiim"),
            vec!["بِسمِ اللَّهِ الرَّحمٰنِ الرَّحيمِ"]
        );
        assert_eq!(
            q.e("alhamdulilla hirobbil 'alamiin"),
            vec!["الحَمدُ لِلَّهِ رَبِّ العٰلَمينَ"]
        );
        assert_eq!(q.e("arrohma nirrohim"), vec!["الرَّحمٰنِ الرَّحيمِ"]);
        assert_eq!(q.e("maliki yau middin"), vec!["مٰلِكِ يَومِ الدّينِ"]);
        assert_eq!(
            q.e("iyyakanakbudu waiyyakanastain"),
            vec!["إِيّاكَ نَعبُدُ وَإِيّاكَ نَستَعينُ"]
        );
        assert_eq!(q.e("ihdinassirotol mustaqim"), vec!["اهدِنَا الصِّرٰطَ المُستَقيمَ"]);
        assert_eq!(
            q.e("shirotolladzina an'amta 'alaihim ghoiril maghdzubi 'alaihim waladdoolliin"),
            vec!["صِرٰطَ الَّذينَ أَنعَمتَ عَلَيهِم غَيرِ المَغضوبِ عَلَيهِم وَلَا الضّالّينَ"]
        );
    }

    #[test]
    fn test_al_ikhlas() {
        let q = Quranize::new(50);
        assert_eq!(q.e("qulhuwallahuahad"), vec!["قُل هُوَ اللَّهُ أَحَدٌ"]);
        assert_eq!(q.e("allahussomad"), vec!["اللَّهُ الصَّمَدُ"]);
        assert_eq!(q.e("lam yalid walam yulad"), vec!["لَم يَلِد وَلَم يولَد"]);
        assert_eq!(
            q.e("walam yakun lahu kufuwan ahad"),
            vec!["وَلَم يَكُن لَهُ كُفُوًا أَحَدٌ"]
        );
    }

    #[test]
    fn test_tajwid() {
        let q = Quranize::new(50);
        assert_eq!(q.e("sami'am bashiro"), vec!["سَميعًا بَصيرًا"]);
        assert_eq!(q.e("mim baini"), vec!["مِن بَينِ"]);
        assert_eq!(q.e("baghyam bainahum"), vec!["بَغيًا بَينَهُم"]);
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
        let texts = q.e("ALLAH");
        let set = std::collections::HashSet::<&String>::from_iter(texts.iter());
        assert_eq!(texts.len(), set.len(), "{:#?}", texts);
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
