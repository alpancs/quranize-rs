//! Encodes alphabetic text to quran text.
//! See [`Quranize`] for details.
//!
//! # Examples
//!
//! ## Adding crate quranize to a project's dependencies
//!
//! Run `cargo add quranize`, or add the following lines to `Cargo.toml` file.
//! ```toml
//! [dependencies]
//! quranize = "0.12"
//! ```
//!
//! ## Encoding alphabetic text to quran text
//!
//! ```
//! let q = quranize::Quranize::new();
//! assert_eq!(q.encode("bismillah").first().unwrap().0, "بِسمِ اللَّه");
//! ```

mod normalization;
mod suffix_tree;
mod transliteration;

use suffix_tree::{Edge, Index};
use transliteration::{contextual_map, harf_muqottoah_map, map};

type EncodeResults = Vec<(String, usize, Vec<&'static str>)>;
type PrevMap = (char, &'static str);

const AYA_COUNT: usize = 6236;
const SURA_STARTS: [usize; 114] = [
    0, 7, 293, 493, 669, 789, 954, 1160, 1235, 1364, 1473, 1596, 1707, 1750, 1802, 1901, 2029,
    2140, 2250, 2348, 2483, 2595, 2673, 2791, 2855, 2932, 3159, 3252, 3340, 3409, 3469, 3503, 3533,
    3606, 3660, 3705, 3788, 3970, 4058, 4133, 4218, 4272, 4325, 4414, 4473, 4510, 4545, 4583, 4612,
    4630, 4675, 4735, 4784, 4846, 4901, 4979, 5075, 5104, 5126, 5150, 5163, 5177, 5188, 5199, 5217,
    5229, 5241, 5271, 5323, 5375, 5419, 5447, 5475, 5495, 5551, 5591, 5622, 5672, 5712, 5758, 5800,
    5829, 5848, 5884, 5909, 5931, 5948, 5967, 5993, 6023, 6043, 6058, 6079, 6090, 6098, 6106, 6125,
    6130, 6138, 6146, 6157, 6168, 6176, 6179, 6188, 6193, 6197, 6204, 6207, 6213, 6216, 6221, 6225,
    6230,
];
const QURAN_TXT: &str = include_str!("quran-uthmani-min.txt");

/// Struct to encode alphabetic text to quran text.
pub struct Quranize {
    tree: suffix_tree::SuffixTree<'static>,
    saqs: Vec<(u8, u16, &'static str)>,
}

impl Quranize {
    const EXPECTED_VERTEX_COUNT: usize = 125_595;

    /// Create a new [`Quranize`] instance.
    ///
    /// # Examples
    ///
    /// ```
    /// let q = quranize::Quranize::new();
    /// assert_eq!(q.encode("bismillah").first().unwrap().0, "بِسمِ اللَّه");
    /// ```
    pub fn new() -> Self {
        let mut tree = suffix_tree::SuffixTree::with_capacity(Self::EXPECTED_VERTEX_COUNT);
        let mut saqs = Vec::with_capacity(AYA_COUNT);
        let mut sura_num = 1;
        (0..AYA_COUNT)
            .zip(QURAN_TXT.split_inclusive('\n'))
            .map(|(i, q)| {
                sura_num += (i == SURA_STARTS.get(sura_num).copied().unwrap_or(AYA_COUNT)) as usize;
                let aya_num = i - SURA_STARTS[sura_num - 1] + 1;
                ((i, sura_num as u8, aya_num as u16), q)
            })
            .map(|((i, s, a), q)| ((i, s, a), Self::trim_basmalah(s, a, q)))
            .for_each(|((i, s, a), q)| {
                tree.construct(i, q);
                saqs.push((s, a, q.trim()));
            });
        Self { tree, saqs }
    }

    fn trim_basmalah(s: u8, a: u16, q: &str) -> &str {
        match (s, a) {
            (1, _) | (9, _) => q,
            (_, 1) => q.splitn(5, ' ').last().unwrap(),
            _ => q,
        }
    }

    pub fn encode(&self, s: &str) -> EncodeResults {
        let mut results: EncodeResults = match normalization::normalize(s).as_str() {
            "" => vec![],
            s => { self.tree.edges_from(0) }
                .flat_map(|&e| self.rev_encode(s, e, None))
                .collect(),
        }
        .into_iter()
        .chain(match normalization::normalize_muqottoah(s).as_str() {
            "" => vec![],
            s => { self.tree.edges_from(0) }
                .flat_map(|&e| self.rev_encode_muqottoah(s, e))
                .collect(),
        })
        .map(|(q, n, e)| (q.chars().rev().collect(), n, e.into_iter().rev().collect()))
        .collect();
        results.dedup_by(|x, y| x.0 == y.0);
        results
    }

    fn rev_encode(&self, s: &str, (v, w, l): Edge, pm: Option<PrevMap>) -> EncodeResults {
        let results_iter = l.chars().next().into_iter().flat_map(|c| -> EncodeResults {
            let tsls = map(c).iter().chain(contextual_map(pm.unzip().0, c));
            let tsl_results_iter = tsls.filter_map(|&tsl| -> Option<EncodeResults> {
                s.strip_prefix(tsl).map(|s| match s {
                    "" => vec![(c.to_string(), self.tree.count_data(w), vec![tsl])],
                    s => match &l[c.len_utf8()..] {
                        "" => { self.tree.edges_from(w) }
                            .flat_map(|&e| self.rev_encode(s, e, Some((c, tsl))))
                            .collect(),
                        l => self.rev_encode(s, (v, w, l), Some((c, tsl))),
                    }
                    .into_iter()
                    .map(|(mut q, n, mut e)| {
                        q.push(c);
                        e.push(tsl);
                        (q, n, e)
                    })
                    .collect(),
                })
            });
            tsl_results_iter.flatten().collect()
        });
        results_iter.collect()
    }

    fn rev_encode_muqottoah(&self, s: &str, (v, w, l): Edge) -> EncodeResults {
        let results_iter = l.chars().next().into_iter().flat_map(|c| -> EncodeResults {
            let tsls = harf_muqottoah_map(c).iter();
            let tsl_results_iter = tsls.filter_map(|&tsl| -> Option<EncodeResults> {
                s.strip_prefix(tsl).map(|s| match s {
                    "" => match self.tree.vertices[w].2 {
                        true => vec![(c.to_string(), self.tree.count_data(w), vec![tsl])],
                        false => vec![],
                    },
                    s => match &l[c.len_utf8()..] {
                        "" => { self.tree.edges_from(w) }
                            .flat_map(|&e| self.rev_encode_muqottoah(s, e))
                            .collect(),
                        l => self.rev_encode_muqottoah(s, (v, w, l)),
                    }
                    .into_iter()
                    .map(|(mut q, n, mut e)| {
                        q.push(c);
                        e.push(tsl);
                        (q, n, e)
                    })
                    .collect(),
                })
            });
            tsl_results_iter.flatten().collect()
        });
        results_iter.collect()
    }

    pub fn find(&self, s: &str) -> Vec<Index> {
        self.tree.find(s, 0)
    }

    pub fn get_sura(&self, i: usize) -> Option<u8> {
        Some(self.saqs.get(i)?.0)
    }

    pub fn get_aya(&self, i: usize) -> Option<u16> {
        Some(self.saqs.get(i)?.1)
    }

    pub fn get_quran(&self, i: usize) -> Option<&str> {
        Some(self.saqs.get(i)?.2)
    }
}

impl Default for Quranize {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    impl Quranize {
        fn e(&self, text: &str) -> Vec<String> {
            self.encode(text).into_iter().map(|r| r.0).collect()
        }
    }

    #[test]
    fn test_quranize_default() {
        let q: Quranize = Default::default();
        assert_eq!(q.e("illa billah"), ["إِلّا بِاللَّه"]);
        assert_eq!(q.e("alqur'an"), ["القُرءان"]);
        assert_eq!(q.e("bismillah"), ["بِسمِ اللَّه"]);
        assert_eq!(q.e("birobbinnas"), ["بِرَبِّ النّاس"]);
        assert_eq!(q.e("inna anzalnahu"), ["إِنّا أَنزَلنٰهُ"]);
        assert_eq!(q.e("wa'tasimu"), ["وَاعتَصِمو"]);
        assert_eq!(q.e("wa'tasimu bihablillah"), ["وَاعتَصِموا بِحَبلِ اللَّه"]);
        assert_eq!(q.e("idza qodho"), ["إِذا قَضَ"]);
        assert_eq!(q.e("masyaallah"), ["ما شاءَ اللَّه"]);
        assert_eq!(q.e("illa man taba"), ["إِلّا مَن تابَ"]);
        assert_eq!(q.e("alla tahzani"), ["أَلّا تَحزَنى"]);
        assert_eq!(q.e("innasya niaka"), ["إِنَّ شانِئَكَ"]);
        assert_eq!(q.e("innasya ni'aka"), ["إِنَّ شانِئَكَ"]);
        assert_eq!(q.e("wasalamun alaihi"), ["وَسَلٰمٌ عَلَيهِ"]);
        assert_eq!(q.e("ulaika hum"), ["أُولٰئِكَ هُم"]);
        assert_eq!(q.e("waladdoollin"), ["وَلَا الضّالّين"]);
        assert_eq!(q.e("undur kaifa"), ["انظُر كَيفَ"]);
        assert_eq!(q.e("lirrohman"), ["لِلرَّحمٰن"]);
        assert_eq!(q.e("waantum muslimun"), ["وَأَنتُم مُسلِمون"]);
        assert_eq!(q.e("laa yukallifullah"), ["لا يُكَلِّفُ اللَّه"]);
        assert_eq!(q.e("robbil alamin"), ["رَبِّ العٰلَمين"]);
        assert_eq!(q.e("husnul maab"), ["حُسنُ المَـٔاب"]);
        assert_eq!(q.e("khusnul ma'ab"), ["حُسنُ المَـٔاب"]);
        assert_eq!(q.e("kufuwan"), ["كُفُوً"]);
        assert_eq!(q.e("yukhodiun"), ["يُخٰدِعون"]);
        assert_eq!(q.e("indallah"), ["عِندَ اللَّه"]);
        assert_eq!(q.e("alimul ghoibi"), ["عٰلِمُ الغَيبِ"]);
        assert_eq!(q.e("kaana dhoifa"), ["كانَ ضَعيفًا"]);
        assert_eq!(q.e("waantum muslimuna"), ["وَأَنتُم مُسلِمونَ"]);
        assert_eq!(q.e("kitabi la roiba"), ["الكِتٰبِ لا رَيبَ"]);
        assert_eq!(q.e("takwili"), ["تَأويلِ"]);
        assert_eq!(q.e("yu'minun"), ["يُؤمِنون"]);
        assert_eq!(q.e("hudan lil muttaqin"), ["هُدًى لِلمُتَّقين"]);
        assert_eq!(q.e("majreeha wamursaha"), ["مَجر۪ىٰها وَمُرسىٰها"]);
        assert_eq!(q.e("fabiayyi alai"), ["فَبِأَىِّ ءالاءِ"]);
    }

    #[test]
    fn test_alfatihah() {
        let q = Quranize::new();
        assert_eq!(q.e("bismillahirrohmanirrohiim"), ["بِسمِ اللَّهِ الرَّحمٰنِ الرَّحيم"]);
        assert_eq!(
            q.e("alhamdulilla hirobbil 'alamiin"),
            ["الحَمدُ لِلَّهِ رَبِّ العٰلَمين"]
        );
        assert_eq!(q.e("arrohma nirrohim"), ["الرَّحمٰنِ الرَّحيم"]);
        assert_eq!(q.e("maliki yau middin"), ["مٰلِكِ يَومِ الدّين"]);
        assert_eq!(
            q.e("iyyakanakbudu waiyyakanastain"),
            ["إِيّاكَ نَعبُدُ وَإِيّاكَ نَستَعين"]
        );
        assert_eq!(q.e("ihdinassirotol mustaqim"), ["اهدِنَا الصِّرٰطَ المُستَقيم"]);
        assert_eq!(
            q.e("shirotolladzina an'amta 'alaihim ghoiril maghdzubi 'alaihim waladdoolliin"),
            ["صِرٰطَ الَّذينَ أَنعَمتَ عَلَيهِم غَيرِ المَغضوبِ عَلَيهِم وَلَا الضّالّين"]
        );
    }

    #[test]
    fn test_al_ikhlas() {
        let q = Quranize::new();
        assert_eq!(q.e("qulhuwallahuahad"), ["قُل هُوَ اللَّهُ أَحَد"]);
        assert_eq!(q.e("allahussomad"), ["اللَّهُ الصَّمَد"]);
        assert_eq!(q.e("lam yalid walam yulad"), ["لَم يَلِد وَلَم يولَد"]);
        assert_eq!(
            q.e("walam yakun lahu kufuwan ahad"),
            ["وَلَم يَكُن لَهُ كُفُوًا أَحَد"]
        );
    }

    #[test]
    fn test_harf_muqottoah() {
        let q = Quranize::new();
        assert_eq!(q.e("alif lam mim"), ["الم"]);
        assert_eq!(q.e("alif laaam miiim"), &["الم"]);
        assert_eq!(q.e("nuun"), &["ن"]);
        assert_eq!(q.e("kaaaf haa yaa aiiin shoood"), &["كهيعص"]);
        assert_eq!(q.e("kaf ha ya 'ain shod"), &["كهيعص"]);
        assert_eq!(q.e("alif lam ro"), &["الر"]);
    }

    #[test]
    fn test_quranize_empty_result() {
        let q = Quranize::new();
        let empty: [String; 0] = [];
        assert_eq!(q.e(""), empty);
        assert_eq!(q.e(" "), empty);
        assert_eq!(q.e(" -"), empty);
        assert_eq!(q.e("abcd"), empty);
        assert_eq!(q.e("1+2=3"), empty);
    }

    #[test]
    fn test_unique() {
        let q = Quranize::new();
        let results = q.e("masyaallah");
        let uresults = std::collections::HashSet::<&String>::from_iter(results.iter());
        let is_unique = results.len() == uresults.len();
        assert!(is_unique, "results are not unique. results: {:#?}", results);
    }

    #[test]
    fn test_tree_find() {
        let q = Quranize::new();
        assert!(q.find("بِسمِ").contains(&(0, 0)));
        assert_eq!(q.find("وَالنّاسِ").last(), Some(&(6235, 28)));
        assert!(q.find("الم").contains(&(7, 0)));
        assert_eq!(q.find("بِسمِ اللَّهِ الرَّحمٰنِ الرَّحيمِ").len(), 2);
        assert!(q.find("").is_empty());
        assert!(q.find("نن").is_empty());
        assert!(q.find("ننن").is_empty());
        assert!(q.find("نننن").is_empty());
        assert!(q.find("2+3+4=9").is_empty());
        assert_eq!(q.find("بِسمِ اللَّهِ الرَّحمٰنِ الرَّحيم").first(), Some(&(0, 0)));
        assert_eq!(q.find("الرَّحمٰنِ الرَّحيم").first(), Some(&(0, 26)));
        assert_eq!(q.find("").first(), None);
        assert_eq!(q.find("abc").first(), None);
    }

    #[test]
    fn test_tree_props() {
        let t = Quranize::new().tree;
        assert_eq!(t.vertices.len(), t.edges.len() + 1);
        assert_eq!(t.count_data(0), t.collect_data(0).len());
        assert_eq!(t.vertices.len(), Quranize::EXPECTED_VERTEX_COUNT);
        assert!(t.vertices[0].2);
        assert!(!t.vertices[Quranize::EXPECTED_VERTEX_COUNT - 1].2);
    }
}
