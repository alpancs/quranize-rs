pub(super) fn normalize(text: &str) -> String {
    text.chars()
        .filter_map(|c| match c.to_ascii_lowercase() {
            c @ ('a'..='z' | '\'') => Some(c),
            _ => None,
        })
        .collect()
}

pub(super) fn normalize_first_aya(text: &str) -> String {
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
        assert_eq!("", normalize_first_aya(""));
        assert_eq!("alif", normalize_first_aya("alif"));
        assert_eq!("lam", normalize_first_aya("laam"));
        assert_eq!("lam", normalize_first_aya("laaam"));
        assert_eq!("lam", normalize_first_aya("laaaam"));
        assert_eq!("lam", normalize_first_aya("laaaam"));
        assert_eq!(
            "kafhayaainshod",
            normalize_first_aya("kaaaf haa yaa aiiin shoood"),
        );
        assert_eq!(
            "kafhaya'ainshod",
            normalize_first_aya("kaaaf haa yaa 'aiiin shoood"),
        );
    }
}
