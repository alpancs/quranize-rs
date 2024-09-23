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
use transliteration::{contextual_map, map};

type EncodeResults = Vec<(String, usize, Vec<&'static str>)>;
type PrevMap = (char, &'static str);

const AYA_COUNT: usize = 6236;
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
            { self.tree.edges_from(0) }
                .flat_map(|&e| self.rev_encode(&s, e, None))
                .collect::<Vec<_>>()
        }
        .into_iter()
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
    fn test_suffix_tree_props() {
        let t = Quranize::new().tree;
        assert_eq!(t.vertices.len(), t.edges.len() + 1);
        assert_eq!(t.count_data(0), t.collect_data(0).len());
        assert_eq!(t.vertices.len(), Quranize::EXPECTED_VERTEX_COUNT);
    }
}
