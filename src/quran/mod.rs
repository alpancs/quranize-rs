mod simple_clean;
mod simple_plain;

pub fn simple_clean_iter() -> impl Iterator<Item = (u8, u16, &'static str)> {
    quran_iter(simple_clean::RAW_QURAN)
}

pub fn simple_plain_iter() -> impl Iterator<Item = (u8, u16, &'static str)> {
    quran_iter(simple_plain::RAW_QURAN)
}

fn quran_iter(raw: &str) -> impl Iterator<Item = (u8, u16, &str)> {
    let raw = raw.trim_start();
    let basmalah = raw.split('\n').next().unwrap().split('|').nth(2).unwrap();
    raw.split('\n').take_while(|l| !l.is_empty()).map(move |l| {
        let mut it = l.split('|');
        let sura_number = it.next().unwrap().parse().unwrap();
        let aya_number = it.next().unwrap().parse().unwrap();
        let mut aya_text = it.next().unwrap();
        if sura_number > 1 && aya_number == 1 {
            aya_text = aya_text.trim_start_matches(basmalah).trim_start();
        }
        (sura_number, aya_number, aya_text)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_quran_versions() {
        for ((s1, a1, t1, c1), (_, _, t2, c2)) in
            word_counts(simple_clean::RAW_QURAN).zip(word_counts(simple_plain::RAW_QURAN))
        {
            assert_eq!(
                c1,c2,
                "sura_number = {s1}, aya_number = {a1},\naya_text = {t1} and {t2},\nword count = {c1} and {c2}",
            );
        }
        assert_same_basmalah(simple_clean::RAW_QURAN);
        assert_same_basmalah(simple_plain::RAW_QURAN);
    }

    fn word_counts(raw: &str) -> impl Iterator<Item = (u8, u16, &str, usize)> {
        quran_iter(raw).map(|(s, a, t)| (s, a, t, t.split_whitespace().count()))
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
                    "sura_number = {sura_number}, aya_number = {aya_number},\naya_text = {aya_text}"
                );
            }
        }
    }
}
