mod simple_clean;
use simple_clean::SIMPLE_CLEAN;

mod simple_plain;
use simple_plain::SIMPLE_PLAIN;

pub fn simple_clean_iter() -> impl Iterator<Item = (u8, u16, &'static str)> {
    quran_iter(SIMPLE_CLEAN)
}

pub fn simple_plain_iter() -> impl Iterator<Item = (u8, u16, &'static str)> {
    quran_iter(SIMPLE_PLAIN)
}

fn quran_iter(raw: &str) -> impl Iterator<Item = (u8, u16, &str)> {
    let raw = raw.trim_start();
    let basmalah = raw.split('\n').next().unwrap().split('|').nth(2).unwrap();
    raw.split('\n').take_while(|l| !l.is_empty()).map(move |l| {
        let mut it = l.split('|');
        (
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().trim_start_matches(basmalah).trim_start(),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_quran_versions() {
        for (wc_a, wc_b) in word_counts(SIMPLE_CLEAN).zip(word_counts(SIMPLE_PLAIN)) {
            assert_eq!(
                wc_a.3, wc_b.3,
                "sura_number = {}, aya_number = {}, aya_text = {} and {}, word count = {} and {}",
                wc_a.0, wc_a.1, wc_a.2, wc_b.2, wc_a.3, wc_b.3
            );
        }
        assert_same_basmalah(SIMPLE_CLEAN);
        assert_same_basmalah(SIMPLE_PLAIN);
    }

    fn word_counts(raw: &str) -> impl Iterator<Item = (u8, u16, &str, usize)> {
        raw.trim_start()
            .split('\n')
            .take_while(|l| !l.is_empty())
            .map(|line| {
                let mut parts = line.split('|');
                let sura_number = parts.next().unwrap().parse().unwrap();
                let aya_number = parts.next().unwrap().parse().unwrap();
                let aya_text = parts.next().unwrap();
                let word_count = aya_text.split_whitespace().count();
                (sura_number, aya_number, aya_text, word_count)
            })
    }

    fn assert_same_basmalah(raw: &str) {
        let mut lines = raw.trim_start().split('\n').take_while(|l| !l.is_empty());
        let basmalah = lines.next().unwrap().split('|').nth(2).unwrap();
        let basmalah = basmalah.to_owned() + " ";
        for line in lines {
            let mut parts = line.split('|');
            let sura_number: u8 = parts.next().unwrap().parse().unwrap();
            let aya_number: u16 = parts.next().unwrap().parse().unwrap();
            let aya_text = parts.next().unwrap();
            if aya_number == 1 && sura_number != 9 {
                assert!(
                    aya_text.starts_with(&basmalah),
                    "sura_number = {}, aya_number = {}, aya_text = {}",
                    sura_number,
                    aya_number,
                    aya_text
                );
            }
        }
    }
}
