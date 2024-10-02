pub(super) fn normalize(text: &str) -> String {
    text.chars()
        .filter_map(|c| match c.to_ascii_lowercase() {
            c @ ('a'..='z' | '\'') => Some(c),
            _ => None,
        })
        .collect()
}

pub(super) fn normalize_muqottoah(text: &str) -> String {
    let mut chars = Vec::from_iter(text.chars().filter_map(|c| match c.to_ascii_lowercase() {
        c @ ('a'..='z' | '\'' | ' ') => Some(c),
        _ => None,
    }));
    chars.dedup_by(|&mut a, &mut b| a == b && (a == 'a' || a == 'o' || a == 'i' || a == 'u'));
    chars.into_iter().filter(|&c| c != ' ').collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_normalize() {
        assert_eq!("", normalize(""));
        assert_eq!("bismi", normalize("bismi"));
        assert_eq!("'aalimulghoibi", normalize("'aalimul ghoibi"));
        assert_eq!("qula'udzu", normalize("Qul A'udzu"));
        assert_eq!(
            "bismillahirrohmanirrohiim",
            normalize("bismilla hirrohma nirrohiim"),
        );
    }

    #[test]
    fn test_normalize_first_aya() {
        assert_eq!("", normalize_muqottoah(""));
        assert_eq!("alif", normalize_muqottoah("alif"));
        assert_eq!("lam", normalize_muqottoah("laam"));
        assert_eq!("lam", normalize_muqottoah("laaam"));
        assert_eq!("lam", normalize_muqottoah("laaaam"));
        assert_eq!("lam", normalize_muqottoah("laaaam"));
        assert_eq!(
            "kafhayaainshod",
            normalize_muqottoah("kaaaf haa yaa aiiin shoood"),
        );
        assert_eq!(
            "kafhaya'ainshod",
            normalize_muqottoah("kaaaf haa yaa 'aiiin shoood"),
        );
    }
}
