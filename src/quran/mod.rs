//! Helper module related to Quran stuffs.

pub(crate) mod harf;

const UTHMANI_MIN: &str = include_str!("quran-uthmani-min.txt");
const SURA_COUNT: usize = 114;
const AYA_COUNT: usize = 6236;
const AYA_STARTS: [usize; 115] = [
    0, 7, 293, 493, 669, 789, 954, 1160, 1235, 1364, 1473, 1596, 1707, 1750, 1802, 1901, 2029,
    2140, 2250, 2348, 2483, 2595, 2673, 2791, 2855, 2932, 3159, 3252, 3340, 3409, 3469, 3503, 3533,
    3606, 3660, 3705, 3788, 3970, 4058, 4133, 4218, 4272, 4325, 4414, 4473, 4510, 4545, 4583, 4612,
    4630, 4675, 4735, 4784, 4846, 4901, 4979, 5075, 5104, 5126, 5150, 5163, 5177, 5188, 5199, 5217,
    5229, 5241, 5271, 5323, 5375, 5419, 5447, 5475, 5495, 5551, 5591, 5622, 5672, 5712, 5758, 5800,
    5829, 5848, 5884, 5909, 5931, 5948, 5967, 5993, 6023, 6043, 6058, 6079, 6090, 6098, 6106, 6125,
    6130, 6138, 6146, 6157, 6168, 6176, 6179, 6188, 6193, 6197, 6204, 6207, 6213, 6216, 6221, 6225,
    6230, 6236,
];

/// Returns an iterator of `(sura_number, aya_number, aya_text)` that iterates each ayah in the Quran.
pub(crate) fn iter() -> impl Iterator<Item = (u8, u16, &'static str)> {
    iter_quran(UTHMANI_MIN)
}

fn iter_quran(raw: &str) -> impl Iterator<Item = (u8, u16, &str)> {
    let mut aya_number = 0u16;
    let mut sura_number = 0u8;
    raw.split('\n')
        .take(AYA_COUNT)
        .enumerate()
        .map(move |(i, aya_text)| {
            if i == AYA_STARTS[sura_number as usize] {
                aya_number = 1;
                sura_number += 1;
            } else {
                aya_number += 1;
            }
            let aya_text = match (sura_number, aya_number) {
                (1, _) | (9, _) => aya_text,
                (_, 1) => aya_text.splitn(5, ' ').last().unwrap(),
                _ => aya_text,
            };
            (sura_number, aya_number, aya_text)
        })
}

/// Struct to get ayah texts by surah number and ayah number.
pub struct AyaGetter<'a> {
    aya_texts: Vec<&'a str>,
    aya_sums: Vec<usize>,
}

impl Default for AyaGetter<'_> {
    fn default() -> Self {
        AyaGetter::new()
    }
}
impl<'a> AyaGetter<'a> {
    /// Create a new `AyaGetter`.
    fn new() -> Self {
        let mut aya_texts = Vec::with_capacity(AYA_COUNT);
        let mut aya_sums = Vec::with_capacity(SURA_COUNT);
        for (i, (_, a, q)) in iter_quran(UTHMANI_MIN).enumerate() {
            aya_texts.push(q);
            if a == 1 {
                aya_sums.push(i);
            }
        }
        Self {
            aya_texts,
            aya_sums,
        }
    }

    /// Get an ayah text given a surah number and an ayah number.
    ///
    /// # Examples
    ///
    /// ```
    /// use quranize::AyaGetter;
    /// let aya_getter = AyaGetter::default();
    /// assert_eq!(aya_getter.get(1, 1), Some("بِسمِ اللَّهِ الرَّحمٰنِ الرَّحيمِ"));
    /// assert_eq!(aya_getter.get(114, 6), Some("مِنَ الجِنَّةِ وَالنّاسِ"));
    /// ```
    pub fn get(&self, sura_number: u8, aya_number: u16) -> Option<&'a str> {
        let aya_sum = *self.aya_sums.get(sura_number as usize - 1)?;
        Some(*self.aya_texts.get(aya_sum + aya_number as usize - 1)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_properties() {
        assert_eq!(iter().count(), AYA_COUNT);
        let unique_unicodes: Vec<_> = {
            let mut set = std::collections::BTreeSet::new();
            for (_, _, t) in iter() {
                set.extend(t.chars());
            }
            set.iter().map(|c| c.escape_unicode().to_string()).collect()
        };
        assert_eq!(
            unique_unicodes,
            [
                "\\u{20}", "\\u{621}", "\\u{623}", "\\u{624}", "\\u{625}", "\\u{626}", "\\u{627}",
                "\\u{628}", "\\u{629}", "\\u{62a}", "\\u{62b}", "\\u{62c}", "\\u{62d}", "\\u{62e}",
                "\\u{62f}", "\\u{630}", "\\u{631}", "\\u{632}", "\\u{633}", "\\u{634}", "\\u{635}",
                "\\u{636}", "\\u{637}", "\\u{638}", "\\u{639}", "\\u{63a}", "\\u{640}", "\\u{641}",
                "\\u{642}", "\\u{643}", "\\u{644}", "\\u{645}", "\\u{646}", "\\u{647}", "\\u{648}",
                "\\u{649}", "\\u{64a}", "\\u{64b}", "\\u{64c}", "\\u{64d}", "\\u{64e}", "\\u{64f}",
                "\\u{650}", "\\u{651}", "\\u{654}", "\\u{670}", "\\u{6dc}", "\\u{6df}", "\\u{6e0}",
                "\\u{6e3}", "\\u{6e5}", "\\u{6e6}", "\\u{6e7}", "\\u{6e8}", "\\u{6ea}", "\\u{6eb}"
            ]
        );
    }

    #[test]
    fn test_map() {
        let aya_getter = AyaGetter::new();
        assert_eq!(aya_getter.get(1, 1), Some("بِسمِ اللَّهِ الرَّحمٰنِ الرَّحيمِ"));
        assert_eq!(aya_getter.get(114, 6), Some("مِنَ الجِنَّةِ وَالنّاسِ"));
        assert_eq!(aya_getter.get(114, 7), None);
    }
}
