mod simple_clean;
pub use simple_clean::SIMPLE_CLEAN;

mod simple_plain;
pub use simple_plain::SIMPLE_PLAIN;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_quran_versions() {
        assert_eq!(count_words(SIMPLE_CLEAN), count_words(SIMPLE_PLAIN));
        assert_eq!(trimmed_basmalah(SIMPLE_PLAIN), 112);
    }

    fn count_words(raw_quran: &str) -> usize {
        raw_quran
            .trim_start()
            .split('\n')
            .take_while(|l| !l.is_empty())
            .map(|l| l.split('|').nth(2).unwrap().split_whitespace().count())
            .sum()
    }

    fn trimmed_basmalah(raw_quran: &str) -> usize {
        let mut lines = raw_quran.trim_start().split('\n');
        let basmalah = lines.next().unwrap().split('|').nth(2).unwrap();
        lines
            .take_while(|l| !l.is_empty())
            .filter_map(|l| l.split('|').nth(2).unwrap().strip_prefix(basmalah))
            .count()
    }
}
