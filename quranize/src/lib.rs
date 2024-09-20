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
//! quranize = "0.11"
//! ```
//!
//! ## Encoding alphabetic text to quran text
//!
//! ```
//! let q = quranize::Quranize::default();
//! assert_eq!(q.encode("alhamdulillah").first().unwrap().0, "الحَمدُ لِلَّهِ");
//! ```
//!
//! ## Getting an aya text given surah number and ayah number
//!
//! ```
//! let aya_getter = quranize::AyaGetter::new();
//! assert_eq!(aya_getter.get(1, 1), Some("بِسمِ اللَّهِ الرَّحمٰنِ الرَّحيمِ"));
//! ```

mod quranize;
pub use quranize::Quranize;

mod quran;
pub use quran::AyaGetter;
