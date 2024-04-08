//! Helper module related to Quran stuffs.

const UTHMANI_MIN: &str = include_str!("quran-uthmani-min.txt");
const AYA_COUNT: u16 = 6236;
const AYA_STARTS: [u16; 115] = [
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
    let mut sura_number = 0u8;
    let mut aya_number = 0u16;
    (0..AYA_COUNT)
        .zip(raw.split('\n'))
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
///
/// # Examples
///
/// ```
/// use quranize::AyaGetter;
/// let aya_getter = AyaGetter::new();
/// assert_eq!(Some("بِسمِ اللَّهِ الرَّحمٰنِ الرَّحيمِ"), aya_getter.get(1, 1));
/// assert_eq!(Some("مِنَ الجِنَّةِ وَالنّاسِ"), aya_getter.get(114, 6));
/// ```
pub struct AyaGetter<'a> {
    aya_texts: Vec<&'a str>,
}

impl Default for AyaGetter<'_> {
    fn default() -> Self {
        AyaGetter::new()
    }
}
impl<'a> AyaGetter<'a> {
    /// Create a new `AyaGetter`.
    pub fn new() -> Self {
        let aya_texts = iter().map(|(_, _, q)| q).collect();
        Self { aya_texts }
    }

    /// Get an ayah text given a surah number and an ayah number.
    pub fn get(&self, sura: u8, aya: u16) -> Option<&'a str> {
        let offset = AYA_STARTS.get(sura as usize - 1)?;
        self.aya_texts.get((offset + aya) as usize - 1).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_properties() {
        assert!(iter().count() == AYA_COUNT.into());
        let unique_unicodes: Vec<_> = {
            let mut set = std::collections::BTreeSet::new();
            for (_, _, t) in iter() {
                set.extend(t.chars());
            }
            set.into_iter().collect()
        };
        assert_eq!(
            [
                '\u{0020}', '\u{0621}', '\u{0623}', '\u{0624}', '\u{0625}', '\u{0626}', '\u{0627}',
                '\u{0628}', '\u{0629}', '\u{062A}', '\u{062B}', '\u{062C}', '\u{062D}', '\u{062E}',
                '\u{062F}', '\u{0630}', '\u{0631}', '\u{0632}', '\u{0633}', '\u{0634}', '\u{0635}',
                '\u{0636}', '\u{0637}', '\u{0638}', '\u{0639}', '\u{063A}', '\u{0640}', '\u{0641}',
                '\u{0642}', '\u{0643}', '\u{0644}', '\u{0645}', '\u{0646}', '\u{0647}', '\u{0648}',
                '\u{0649}', '\u{064A}', '\u{064B}', '\u{064C}', '\u{064D}', '\u{064E}', '\u{064F}',
                '\u{0650}', '\u{0651}', '\u{0654}', '\u{0670}', '\u{06D6}', '\u{06D7}', '\u{06D8}',
                '\u{06D9}', '\u{06DA}', '\u{06DB}', '\u{06DC}', '\u{06DF}', '\u{06E0}', '\u{06E3}',
                '\u{06E5}', '\u{06E6}', '\u{06E7}', '\u{06E8}', '\u{06EA}', '\u{06EB}'
            ],
            *unique_unicodes
        );
    }

    #[test]
    fn test_map() {
        let aya_getter = AyaGetter::new();
        assert_eq!(Some("بِسمِ اللَّهِ الرَّحمٰنِ الرَّحيمِ"), aya_getter.get(1, 1));
        assert_eq!(Some("مِنَ الجِنَّةِ وَالنّاسِ"), aya_getter.get(114, 6));
        assert_eq!(None, aya_getter.get(114, 7));
    }
}
