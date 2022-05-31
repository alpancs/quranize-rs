mod harf;
pub use harf::{Harf, Location};

mod quran_simple_clean;

pub fn build_quran_index(word_count_limit: u8) -> Harf {
    let mut root = Harf::new('\0');
    let lines = quran_simple_clean::RAW.trim_start().split('\n');
    for line in lines.take_while(|l| !l.is_empty()) {
        let mut splitted_line = line.split('|');
        let sura_number: u8 = splitted_line.next().unwrap().parse().unwrap();
        let aya_number: u16 = splitted_line.next().unwrap().parse().unwrap();
        let mut aya_text = splitted_line.next().unwrap();
        if aya_number == 1 && !(sura_number == 1 || sura_number == 9) {
            aya_text = aya_text.strip_prefix("بسم الله الرحمن الرحيم ").unwrap();
        }
        root.update_tree(sura_number, aya_number, aya_text, word_count_limit);
    }
    root
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_quran_index() {
        let quran_index = build_quran_index(u8::MAX);
        assert_eq!(quran_index.content, '\0');
        assert_eq!(quran_index.next_harfs.len(), 31);

        let ba = quran_index
            .next_harfs
            .iter()
            .find(|h| h.content == 'ب')
            .unwrap();
        assert!(ba.locations.is_empty());

        let nun = quran_index
            .next_harfs
            .iter()
            .find(|h| h.content == 'ن')
            .unwrap();
        assert_eq!(nun.locations, vec![Location::new(68, 1, 1)]);
    }
}
