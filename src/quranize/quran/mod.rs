mod simple_clean;
pub use simple_clean::SIMPLE_CLEAN;

mod simple_plain;
pub use simple_plain::SIMPLE_PLAIN;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_quran_versions() {
        assert_eq!(word_counts(SIMPLE_CLEAN), word_counts(SIMPLE_PLAIN));
        assert_same_basmalah(SIMPLE_CLEAN);
        assert_same_basmalah(SIMPLE_PLAIN);
    }

    fn word_counts(raw: &str) -> Vec<usize> {
        raw.trim_start()
            .split('\n')
            .take_while(|l| !l.is_empty())
            .map(|l| l.split('|').nth(2).unwrap().split_whitespace().count())
            .collect()
    }

    fn assert_same_basmalah(raw: &str) {
        let mut lines = raw.trim_start().split('\n').take_while(|l| !l.is_empty());
        let basmalah = lines.next().unwrap().split('|').nth(2).unwrap();
        for line in lines {
            let mut parts = line.split('|');
            let sura_number: u8 = parts.next().unwrap().parse().unwrap();
            let aya_number: u16 = parts.next().unwrap().parse().unwrap();
            let aya_text = parts.next().unwrap();
            if aya_number == 1 && sura_number != 9 {
                assert!(aya_text.starts_with(basmalah))
            }
        }
    }
}
