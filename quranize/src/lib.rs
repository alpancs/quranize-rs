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

use suffix_tree::Edge;
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
}

impl Quranize {
    const EXPECTED_VERTEX_COUNT: usize = 126_327;

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
        (0..AYA_COUNT)
            .zip(QURAN_TXT.split_inclusive('\n'))
            .for_each(|(i, s)| tree.construct(i, s));
        Self { tree }
    }

    pub fn encode(&self, s: &str) -> EncodeResults {
        {
            let s = normalization::normalize(s);
            self.tree
                .edges_from(0)
                .flat_map(|&e| self.rev_encode(&s, e, None))
                .collect::<Vec<_>>()
        }
        .into_iter()
        .chain({
            let s = normalization::normalize_muqottoah(s);
            self.tree
                .edges_from(0)
                .flat_map(|&e| self.rev_encode_muqottoah(&s, e))
                .collect::<Vec<_>>()
        })
        .map(|(q, n, e)| (q.chars().rev().collect(), n, e.into_iter().rev().collect()))
        .collect()
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
    fn test_alfatihah() {
        let q = Quranize::new();
        assert_eq!(q.e("bismillahirrohmanirrohiim"), ["بِسمِ اللَّهِ الرَّحمٰنِ الرَّحيم"]);
    }

    #[test]
    fn test_muqottoah() {
        let q = Quranize::new();
        assert_eq!(q.e("alif lam mim"), ["الم"]);
        assert_eq!(q.e("alif laaam miiim"), &["الم"]);
        assert_eq!(q.e("nuun"), &["ن"]);
        assert_eq!(q.e("kaaaf haa yaa aiiin shoood"), &["كهيعص"]);
        assert_eq!(q.e("kaf ha ya 'ain shod"), &["كهيعص"]);
    }

    #[test]
    fn test_suffix_tree_props() {
        let t = Quranize::new().tree;
        assert_eq!(t.vertices.len(), t.edges.len() + 1);
        assert_eq!(t.count_data(0), t.collect_data(0).len());
        assert_eq!(t.vertices.len(), Quranize::EXPECTED_VERTEX_COUNT);
        assert!(t.vertices[0].2);
    }
}
