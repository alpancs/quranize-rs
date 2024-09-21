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

mod suffix_tree;

type EncodeResults = Vec<(String, Vec<&'static str>)>;

pub(crate) const QURAN_UTHMANI_MIN: &str = include_str!("quran-uthmani-min.txt");

/// Struct to encode alphabetic text to quran text.
pub struct Quranize {
    tree: suffix_tree::SuffixTree<'static>,
}

impl Quranize {
    /// Create a new [`Quranize`] instance.
    ///
    /// # Examples
    ///
    /// ```
    /// let q = quranize::Quranize::new();
    /// assert_eq!(q.encode("bismillah").first().unwrap().0, "بِسمِ اللَّه");
    /// ```
    pub fn new() -> Self {
        let s = &QURAN_UTHMANI_MIN[..(QURAN_UTHMANI_MIN.find("\n\n").unwrap() + 2)];
        let tree = suffix_tree::SuffixTree::new(s);
        Self { tree }
    }

    pub fn encode(&self, _text: &str) -> EncodeResults {
        vec![]
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
            self.encode(text).into_iter().map(|(q, _)| q).collect()
        }
    }

    #[test]
    fn test_alfatihah() {
        let q = Quranize::new();
        assert_eq!(q.e("bismillahirrohmanirrohiim"), ["بِسمِ اللَّهِ الرَّحمٰنِ الرَّحيم"]);
    }
}
