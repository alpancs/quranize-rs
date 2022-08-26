use crate::quran::{quran_iter, AYA_COUNT, SIMPLE_PLAIN, SURA_COUNT};

pub struct Map {
    aya_texts: [&'static str; AYA_COUNT],
    aya_sums: [usize; SURA_COUNT],
}

pub fn build_map() -> Map {
    let mut aya_texts = [""; AYA_COUNT];
    let mut aya_sums = [0; SURA_COUNT];
    for (i, (s, _, t)) in quran_iter(SIMPLE_PLAIN).enumerate() {
        let s = s as usize;
        aya_texts[i] = t;
        if s < SURA_COUNT {
            aya_sums[s] = i + 1;
        }
    }
    Map {
        aya_texts,
        aya_sums,
    }
}

impl Map {
    pub fn get(&self, sura_number: u8, aya_number: u16) -> Option<&'static str> {
        let aya_sum = *self.aya_sums.get(sura_number as usize - 1)?;
        Some(*self.aya_texts.get(aya_sum + aya_number as usize - 1)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let map = build_map();
        assert_eq!(map.get(1, 1), Some("بِسْمِ اللَّهِ الرَّحْمَـٰنِ الرَّحِيمِ"));
        assert_eq!(map.get(114, 6), Some("مِنَ الْجِنَّةِ وَالنَّاسِ"));
        assert_eq!(map.get(114, 7), None);
    }
}
