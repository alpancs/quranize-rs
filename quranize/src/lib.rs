//! [Quranize] encodes alphabetic text into quran text, a.k.a. transliteration.
//!
//! # Examples
//!
//! ## Adding crate quranize to a project's dependencies
//!
//! Run `cargo add quranize`, or add the following lines to `Cargo.toml` file.
//! ```toml
//! [dependencies]
//! quranize = "1.0"
//! ```
//!
//! ## Encoding alphabetic text to quran text
//!
//! ```
//! let q = quranize::Quranize::new();
//!
//! assert_eq!(q.encode("bismillahirrohmanirrohim")[0].0, "بِسمِ اللَّهِ الرَّحمـٰنِ الرَّحيم");
//! assert_eq!(q.encode("amma yatasa alun")[0].0, "عَمَّ يَتَساءَلون");
//!
//! let (i, _) = q.find("عَمَّ يَتَساءَلون")[0];
//! let &(page, sura, aya, _) = q.get_data(i).unwrap();
//! assert_eq!((page, sura, aya), (582, 78, 1));
//! ```

mod normalization;
mod quran_metadata;
mod suffix_tree;
mod transliteration;

use suffix_tree::{Edge, Index, SuffixTree};
use transliteration::{contextual_map, harf_muqottoah_map, map};

type EncodeResults = Vec<(String, usize, Vec<&'static str>)>;
type PrevMap = (char, &'static str);

use quran_metadata::*;
const QURAN_TXT: &str = include_str!("quran-simple-min.txt");

/// Quranize model, for doing transliteration, finding string, and getting aya.
pub struct Quranize {
    data: Vec<(u16, u8, u16, &'static str)>,
    tree: suffix_tree::SuffixTree<'static>,
}

impl Quranize {
    const EXPECTED_VERTEX_COUNT: usize = 126_307;

    /// Create a new [`Quranize`] instance.
    pub fn new() -> Self {
        let mut data = Vec::with_capacity(AYA_COUNT);
        let mut tree = SuffixTree::with_capacity(Self::EXPECTED_VERTEX_COUNT);

        let mut sura_num = 0;
        let mut page = 0;
        (0..AYA_COUNT)
            .zip(QURAN_TXT.split_inclusive('\n'))
            .map(|(i, q)| {
                sura_num += (SURA_STARTS.get(sura_num) == Some(&i)) as usize;
                let aya_num = i - SURA_STARTS[sura_num - 1] + 1;
                page += (PAGE_OFFSETS.get(page) == Some(&(sura_num, aya_num))) as usize;
                (i, page as u16, sura_num as u8, aya_num as u16, q)
            })
            .map(|(i, p, s, a, q)| (i, p, s, a, Self::trim_basmalah(s, a, q)))
            .for_each(|(i, p, s, a, q)| {
                data.push((p, s, a, q.trim_end()));
                tree.construct(i, q);
            });
        Self { data, tree }
    }

    fn trim_basmalah(s: u8, a: u16, q: &str) -> &str {
        match (s, a) {
            (1, _) | (9, _) => q,
            (_, 1) => q.splitn(5, ' ').last().unwrap(),
            _ => q,
        }
    }

    /// Do transliteration on `s`, returning a list of tuple:
    /// - `String`: transliteration result / quran form
    /// - `usize`: location count where the quran form above is found in Alquran
    /// - `Vec<&'static str>`: explanation for each chars in the quran form above
    ///
    /// # Examples
    ///
    /// ```
    /// let q = quranize::Quranize::new();
    /// assert_eq!(q.encode("alif lam mim"), [("الم".to_string(), 912, vec!["alif", "lam", "mim"])]);
    /// assert_eq!(q.encode("minal jinnati wannas")[0].0, "مِنَ الجِنَّةِ وَالنّاس");
    /// ```
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

    /// Find `s` in Alquran, returning a list of `Index`, where
    /// `Index` is a tuple, containing:
    /// - `usize`: aya row / aya offset (`0..6236`)
    /// - `usize`: string offset in a specific aya (`0..length of aya`)
    ///
    /// # Examples
    /// ```
    /// let q = quranize::Quranize::new();
    /// let index = q.find("عَمَّ يَتَساءَلون")[0];
    /// assert_eq!(index, (5672, 0));
    /// ```
    pub fn find(&self, s: &str) -> Vec<Index> {
        self.tree.find(s, 0)
    }

    /// Get the data for a specific aya row / aya offset (`i`: 0..6236).
    /// Data is a tuple of:
    /// - `u16`: page number
    /// - `u8`: sura number
    /// - `u16`: aya number
    /// - `&str`: aya text
    ///
    /// # Examples
    /// ```
    /// let q = quranize::Quranize::new();
    /// assert_eq!(q.get_data(5672), Some(&(582, 78, 1, "عَمَّ يَتَساءَلونَ")));
    /// ```
    pub fn get_data(&self, i: usize) -> Option<&(u16, u8, u16, &str)> {
        self.data.get(i)
    }

    /// Get the data for a specific page number (`page`: 1..604).
    /// Returns a vector of tuples, each tuple contains:
    /// - `u16`: page number
    /// - `u8`: sura number
    /// - `u16`: aya number
    /// - `&str`: aya text
    /// # Examples
    /// ```
    /// let q = quranize::Quranize::new();
    /// let page_data = q.get_data_from_page(582).unwrap();
    /// assert_eq!(page_data.len(), 30);
    /// assert_eq!(page_data[0], &(582, 78, 1, "عَمَّ يَتَساءَلونَ"));
    /// ```
    pub fn get_data_from_page(&self, page: u16) -> Option<Vec<&(u16, u8, u16, &str)>> {
        let same_page = |&&(p, _, _, _): &&(u16, u8, u16, &str)| p == page;
        let pos = { self.data.binary_search_by_key(&page, |&(p, _, _, _)| p) }.ok()?;
        let left_item_count = self.data[..pos].iter().rev().take_while(same_page).count();
        let left_pos = pos - left_item_count;
        let page_data = self.data[left_pos..].iter().take_while(same_page).collect();
        Some(page_data)
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
        assert_eq!(q.e("alqur'an"), ["القُرآن"]);
        assert_eq!(q.e("bismillah"), ["بِسمِ اللَّه"]);
        assert_eq!(q.e("birobbinnas"), ["بِرَبِّ النّاس"]);
        assert_eq!(q.e("inna anzalnahu"), ["إِنّا أَنزَلناهُ"]);
        assert_eq!(q.e("wa'tasimu"), ["وَاعتَصِمو"]);
        assert_eq!(q.e("wa'tasimu bihablillah"), ["وَاعتَصِموا بِحَبلِ اللَّه"]);
        assert_eq!(q.e("idza qodho"), ["إِذا قَضَ"]);
        assert_eq!(q.e("masyaallah"), ["ما شاءَ اللَّه"]);
        assert_eq!(q.e("illa man taba"), ["إِلّا مَن تابَ"]);
        assert_eq!(q.e("alla tahzani"), ["أَلّا تَحزَني"]);
        assert_eq!(q.e("innasya niaka"), ["إِنَّ شانِئَكَ"]);
        assert_eq!(q.e("innasya ni'aka"), ["إِنَّ شانِئَكَ"]);
        assert_eq!(q.e("wasalamun alaihi"), ["وَسَلامٌ عَلَيهِ"]);
        assert_eq!(q.e("ulaika hum"), ["أُولـٰئِكَ هُم"]);
        assert_eq!(q.e("waladdoollin"), ["وَلَا الضّالّين"]);
        assert_eq!(q.e("undur kaifa"), ["انظُر كَيفَ"]);
        assert_eq!(q.e("lirrohman"), ["لِلرَّحمـٰن"]);
        assert_eq!(q.e("waantum muslimun"), ["وَأَنتُم مُسلِمون"]);
        assert_eq!(q.e("laa yukallifullah"), ["لا يُكَلِّفُ اللَّه"]);
        assert_eq!(q.e("robbil alamin"), ["رَبِّ العالَمين"]);
        assert_eq!(q.e("husnul maab"), ["حُسنُ المَآب"]);
        assert_eq!(q.e("khusnul ma'ab"), ["حُسنُ المَآب"]);
        assert_eq!(q.e("kufuwan"), ["كُفُوً"]);
        assert_eq!(q.e("yukhodiun"), ["يُخادِعون"]);
        assert_eq!(q.e("indallah"), ["عِندَ اللَّه"]);
        assert_eq!(q.e("alimul ghoibi"), ["عالِمُ الغَيبِ"]);
        assert_eq!(q.e("kaana dhoifa"), ["كانَ ضَعيفًا"]);
        assert_eq!(q.e("waantum muslimuna"), ["وَأَنتُم مُسلِمونَ"]);
        assert_eq!(q.e("kitabi la roiba"), ["الكِتابِ لا رَيبَ"]);
        assert_eq!(q.e("takwili"), ["تَأويلِ"]);
        assert_eq!(q.e("yu'minun"), ["يُؤمِنون"]);
        assert_eq!(q.e("hudan lil muttaqin"), ["هُدًى لِلمُتَّقين"]);
        assert_eq!(q.e("majreeha wamursaha"), ["مَجراها وَمُرساها"]);
        assert_eq!(q.e("fabiayyi alai"), ["فَبِأَيِّ آلاءِ"]);
        assert_eq!(q.e("wayuallimukumma"), ["وَيُعَلِّمُكُم ما"]);
        assert_eq!(q.e("wassolat"), ["وَالصَّلاة"]);
    }

    #[test]
    fn test_alfatihah() {
        let q = Quranize::new();
        assert_eq!(
            q.e("bismillahirrohmanirrohiim"),
            ["بِسمِ اللَّهِ الرَّحمـٰنِ الرَّحيم"]
        );
        assert_eq!(
            q.e("alhamdulilla hirobbil 'alamiin"),
            ["الحَمدُ لِلَّهِ رَبِّ العالَمين"]
        );
        assert_eq!(q.e("arrohma nirrohim"), ["الرَّحمـٰنِ الرَّحيم"]);
        assert_eq!(q.e("maliki yau middin"), ["مالِكِ يَومِ الدّين"]);
        assert_eq!(
            q.e("iyyakanakbudu waiyyakanastain"),
            ["إِيّاكَ نَعبُدُ وَإِيّاكَ نَستَعين"]
        );
        assert_eq!(q.e("ihdinassirotol mustaqim"), ["اهدِنَا الصِّراطَ المُستَقيم"]);
        assert_eq!(
            q.e("shirotolladzina an'amta 'alaihim ghoiril maghdzubi 'alaihim waladdoolliin"),
            ["صِراطَ الَّذينَ أَنعَمتَ عَلَيهِم غَيرِ المَغضوبِ عَلَيهِم وَلَا الضّالّين"]
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
        assert!(is_unique, "results are not unique. results: {results:#?}");
    }

    #[test]
    fn test_tree_find() {
        let q = Quranize::new();
        assert!(q.find("بِسمِ").contains(&(0, 0)));
        assert_eq!(q.find("وَالنّاسِ").last(), Some(&(6235, 28)));
        assert!(q.find("الم").contains(&(7, 0)));
        assert_eq!(q.find("بِسمِ اللَّهِ الرَّحمـٰنِ الرَّحيمِ").len(), 2);
        assert!(q.find("").is_empty());
        assert!(q.find("نن").is_empty());
        assert!(q.find("ننن").is_empty());
        assert!(q.find("نننن").is_empty());
        assert!(q.find("2+3+4=9").is_empty());
        assert_eq!(q.find("بِسمِ اللَّهِ الرَّحمـٰنِ الرَّحيمِ").first(), Some(&(0, 0)));
        assert_eq!(q.find("الرَّحمـٰنِ الرَّحيمِ").first(), Some(&(0, 26)));
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
