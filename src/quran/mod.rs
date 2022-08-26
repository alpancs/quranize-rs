mod simple_clean;
mod simple_plain;

pub use simple_clean::RAW_QURAN as SIMPLE_CLEAN;
pub use simple_plain::RAW_QURAN as SIMPLE_PLAIN;

pub const SURA_COUNT: usize = 114;
pub const AYA_COUNT: usize = 6236;

pub fn quran_iter(raw: &str) -> impl Iterator<Item = (u8, u16, &str)> {
    let raw = raw.trim_start();
    let basmalah = raw.split('\n').next().unwrap().split('|').nth(2).unwrap();
    let basmalah_prefix = basmalah.to_string() + " ";
    raw.split('\n').take(AYA_COUNT).map(move |l| {
        let mut it = l.split('|');
        let sura_number = it.next().unwrap().parse().unwrap();
        let aya_number = it.next().unwrap().parse().unwrap();
        let mut aya_text = it.next().unwrap();
        if sura_number != 1 && sura_number != 9 && aya_number == 1 {
            aya_text = aya_text.strip_prefix(&basmalah_prefix).unwrap();
        }
        (sura_number, aya_number, aya_text)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quran_version_compatibility() {
        let word_counter =
            |(s, a, t): (u8, u16, &'static str)| (s, a, t, t.split_whitespace().count());
        let simple_clean_wc_iter = quran_iter(SIMPLE_CLEAN).map(word_counter);
        let simple_plain_wc_iter = quran_iter(SIMPLE_PLAIN).map(word_counter);
        for ((s1, a1, t1, c1), (_, _, t2, c2)) in simple_clean_wc_iter.zip(simple_plain_wc_iter) {
            assert_eq!(c1, c2, "sura_number = {s1}, aya_number = {a1},\naya_text = {t1} and {t2},\nword count = {c1} and {c2}");
        }
    }

    #[test]
    fn test_properties() {
        assert_same_basmalah(simple_clean::RAW_QURAN);
        assert_same_basmalah(simple_plain::RAW_QURAN);
        assert_eq!(quran_iter(SIMPLE_CLEAN).count(), AYA_COUNT);
        assert_eq!(quran_iter(SIMPLE_PLAIN).count(), AYA_COUNT);
    }

    fn assert_same_basmalah(raw: &str) {
        let mut lines = raw.trim_start().split('\n').take(AYA_COUNT);
        let basmalah = lines.next().unwrap().split('|').nth(2).unwrap();
        let basmalah = basmalah.to_string() + " ";
        for line in lines {
            let mut parts = line.split('|');
            let sura_number: u8 = parts.next().unwrap().parse().unwrap();
            let aya_number: u16 = parts.next().unwrap().parse().unwrap();
            let aya_text = parts.next().unwrap();
            if aya_number == 1 && sura_number != 9 {
                assert!(
                    aya_text.starts_with(&basmalah),
                    "sura_number = {sura_number}, aya_number = {aya_number},\naya_text = {aya_text}"
                );
            }
        }
    }
}
