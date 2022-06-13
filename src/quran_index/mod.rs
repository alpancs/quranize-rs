use std::collections::HashMap;

mod harf;
pub use harf::Harf;

mod quran_simple_clean;
mod quran_simple_enhanched;

pub type AyaMap = HashMap<(u8, u16), String>;

pub fn build_quran_index(word_count_limit: u8) -> Harf {
    let mut root = Harf::new('\0');
    let mut basmalah = String::new();
    for line in split_line_quran(quran_simple_clean::RAW) {
        let (sura_number, aya_number, aya_text) = split_aya_line(line, &basmalah);
        root.update_tree(sura_number, aya_number, aya_text, word_count_limit);
        if sura_number == 1 && aya_number == 1 {
            basmalah = aya_text.to_owned() + " ";
        }
    }
    root
}

pub fn build_aya_map() -> AyaMap {
    let mut aya_map = HashMap::new();
    let mut basmalah = String::new();
    for line in split_line_quran(quran_simple_enhanched::RAW) {
        let (sura_number, aya_number, aya_text) = split_aya_line(line, &basmalah);
        aya_map.insert((sura_number, aya_number), String::from(aya_text));
        if sura_number == 1 && aya_number == 1 {
            basmalah = aya_text.to_owned() + " ";
        }
    }
    aya_map
}

fn split_line_quran(raw: &str) -> impl Iterator<Item = &str> {
    raw.trim_start().split('\n').take_while(|l| !l.is_empty())
}

fn split_aya_line<'a>(line: &'a str, basmalah: &str) -> (u8, u16, &'a str) {
    let mut line_parts = line.split('|');
    let sura_number = line_parts.next().unwrap().parse().unwrap();
    let aya_number = line_parts.next().unwrap().parse().unwrap();
    let aya_text = line_parts.next().unwrap().trim_start_matches(basmalah);
    (sura_number, aya_number, aya_text)
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
        assert_eq!(nun.locations, vec![(68, 1, 1)]);
    }

    #[test]
    fn validate_quran_versions() {
        let clean_word_count: usize = quran_simple_clean::RAW
            .trim_start()
            .split('\n')
            .take_while(|l| !l.is_empty())
            .map(|l| l.split('|').nth(2).unwrap().split_whitespace().count())
            .sum();
        let enhanched_word_count: usize = quran_simple_enhanched::RAW
            .trim_start()
            .split('\n')
            .take_while(|l| !l.is_empty())
            .map(|l| l.split('|').nth(2).unwrap().split_whitespace().count())
            .sum();
        assert_eq!(clean_word_count, enhanched_word_count);
    }
}
