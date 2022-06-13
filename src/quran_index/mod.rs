use std::collections::HashMap;

mod harf;
pub use harf::Harf;

mod quran_simple_clean;
mod quran_simple_enhanched;

pub type AyaMap = HashMap<(u8, u16), String>;

pub fn build_quran_index(word_count_limit: u8) -> Harf {
    let mut root = Harf::new('\0');
    for (sura_number, aya_number, aya_text) in get_aya_iterator(quran_simple_clean::RAW) {
        root.update_tree(sura_number, aya_number, aya_text, word_count_limit);
    }
    root
}

pub fn build_aya_map() -> AyaMap {
    let mut aya_map = HashMap::new();
    for (sura_number, aya_number, aya_text) in get_aya_iterator(quran_simple_enhanched::RAW) {
        aya_map.insert((sura_number, aya_number), aya_text.to_owned());
    }
    aya_map
}

fn get_aya_iterator(raw: &str) -> impl Iterator<Item = (u8, u16, &str)> {
    let raw = raw.trim_start();
    let basmalah = raw.split('\n').next().unwrap().split('|').nth(2).unwrap();
    let basmalah = basmalah.to_owned() + " ";
    raw.split('\n')
        .take_while(|l| !l.is_empty())
        .map(move |l| split_aya_line(l, &basmalah))
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
        assert_eq!(
            count_words(quran_simple_clean::RAW),
            count_words(quran_simple_enhanched::RAW)
        );
    }

    fn count_words(quran: &str) -> usize {
        quran
            .trim_start()
            .split('\n')
            .take_while(|l| !l.is_empty())
            .map(|l| l.split('|').nth(2).unwrap().split_whitespace().count())
            .sum()
    }
}
