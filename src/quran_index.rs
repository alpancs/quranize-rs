mod harf;
pub use harf::Harf;

mod quran_simple_clean;

pub fn build_quran_index() -> Harf {
    let mut root = Harf::new('\0');
    let lines = quran_simple_clean::RAW.trim_start().split('\n');
    for line in lines.take_while(|l| !l.is_empty()) {
        let mut splitted_line = line.split('|');
        let sura_number: u8 = splitted_line.next().unwrap().parse().unwrap();
        let aya_number: u16 = splitted_line.next().unwrap().parse().unwrap();
        let mut aya_text = splitted_line.next().unwrap();
        if (sura_number, aya_number) != (1, 1) {
            aya_text = aya_text.trim_start_matches("بسم الله الرحمن الرحيم ");
        }
        root.update_tree(sura_number, aya_number, aya_text);
    }
    root
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test;
    use test::Bencher;

    #[test]
    fn test_build_quran_index() {
        let quran_index = build_quran_index();
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
        assert_eq!(nun.locations.len(), 1);
    }

    #[bench]
    fn bench_build_quran_index(b: &mut Bencher) {
        b.iter(build_quran_index);
    }
}
