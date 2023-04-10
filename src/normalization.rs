pub(crate) fn normalize(text: &str) -> String {
    let chars = Vec::from_iter(text.chars().filter_map(|c| match c.to_ascii_lowercase() {
        c @ ('a'..='z' | '\'' | ' ') => Some(c),
        _ => None,
    }));
    chars.into_iter().filter(|&c| c != ' ').collect()
}

pub(crate) fn normalize_first_aya(text: &str) -> String {
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
    #[test]
    fn test_normalize() {
        assert_eq!(normalize(""), "");
        assert_eq!(normalize("bismi"), "bismi");
        assert_eq!(normalize("'aalimul ghoibi"), "'aalimulghoibi");
        assert_eq!(normalize("Qul A'udzu"), "qula'udzu");
        assert_eq!(
            normalize("bismilla hirrohma nirrohiim"),
            "bismillahirrohmanirrohiim"
        );
    }

    #[test]
    fn test_normalize_first_aya() {
        assert_eq!(normalize_first_aya(""), "");
        assert_eq!(normalize_first_aya("alif"), "alif");
        assert_eq!(normalize_first_aya("laam"), "lam");
        assert_eq!(normalize_first_aya("laaam"), "lam");
        assert_eq!(normalize_first_aya("laaaam"), "lam");
        assert_eq!(normalize_first_aya("laaaam"), "lam");
        assert_eq!(
            normalize_first_aya("kaaaf haa yaa aiiin shoood"),
            "kafhayaainshod"
        );
        assert_eq!(
            normalize_first_aya("kaaaf haa yaa 'aiiin shoood"),
            "kafhaya'ainshod"
        );
    }
}
