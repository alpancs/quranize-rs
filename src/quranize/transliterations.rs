use crate::quran::harf::*;

pub(super) fn mappable(c: char) -> bool {
    c == SHADDA || !map(c).is_empty()
}

pub(super) fn map(c: char) -> &'static [&'static str] {
    match c {
        SPACE => &[""],

        LETTER_HAMZA => &["'", "a"],
        LETTER_ALEF_WITH_MADDA_ABOVE => &["a", "aa"],
        LETTER_ALEF_WITH_HAMZA_ABOVE => &["a", "u"],
        LETTER_WAW_WITH_HAMZA_ABOVE => &["u"],
        LETTER_ALEF_WITH_HAMZA_BELOW => &["i"],
        LETTER_YEH_WITH_HAMZA_ABOVE => &["'", "a", "i"],
        LETTER_ALEF => &["a", "aa", "o", "oo"],
        LETTER_BEH => &["b"],
        LETTER_TEH_MARBUTA => &["h", "t"],
        LETTER_TEH => &["t"],
        LETTER_THEH => &["ts", "s"],
        LETTER_JEEM => &["j"],
        LETTER_HAH => &["h", "ch"],
        LETTER_KHAH => &["kh"],
        LETTER_DAL => &["d"],
        LETTER_THAL => &["d", "dh", "dz"],
        LETTER_REH => &["r"],
        LETTER_ZAIN => &["z"],
        LETTER_SEEN => &["s"],
        LETTER_SHEEN => &["s", "sy", "sh"],
        LETTER_SAD => &["s", "sh"],
        LETTER_DAD => &["d", "dh", "dz"],
        LETTER_TAH => &["t", "th"],
        LETTER_ZAH => &["d", "dh", "dz"],
        LETTER_AIN => &["'", "a", "u", "i", "k"],
        LETTER_GHAIN => &["g", "gh"],

        TATWEEL => &["a", "o"],

        LETTER_FEH => &["f"],
        LETTER_QAF => &["k", "q"],
        LETTER_KAF => &["k"],
        LETTER_LAM => &["l"],
        LETTER_MEEM => &["m"],
        LETTER_NOON => &["n"],
        LETTER_HEH => &["h"],
        LETTER_WAW => &["w", "u", "uu"],
        LETTER_ALEF_MAKSURA => &["a", "o", "i"],
        LETTER_YEH => &["y", "i", "ii"],

        FATHATAN => &["an", "on", ""],
        DAMMATAN => &["un", ""],
        KASRATAN => &["in", ""],
        FATHA => &["a", "o", ""],
        DAMMA => &["u", ""],
        KASRA => &["i", ""],

        HAMZA_ABOVE => &["'", "a"],
        LETTER_SUPERSCRIPT_ALEF => &["a", "aa", "o", "oo"],

        _ => &[],
    }
}

pub(super) fn contextual_map(c0: char, c1: char) -> &'static [&'static str] {
    match (c0, c1) {
        (SPACE | LETTER_HAMZA | LETTER_WAW | FATHATAN | KASRA | HAMZA_ABOVE, LETTER_ALEF)
        | (LETTER_ALEF | KASRA, LETTER_LAM)
        | (LETTER_AIN, LETTER_WAW | LETTER_YEH | LETTER_SUPERSCRIPT_ALEF)
        | (DAMMA, LETTER_WAW) => &[""],
        ('\0', LETTER_ALEF) => &["u", "i", ""],
        (_, SHADDA) => map(c0),
        _ => &[],
    }
}

pub(super) fn single_harf_map(c: char) -> &'static [&'static str] {
    match c {
        LETTER_ALEF => &["alif"],
        LETTER_BEH => &["ba"],
        LETTER_TEH => &["ta"],
        LETTER_THEH => &["tsa", "sa"],
        LETTER_JEEM => &["jim"],
        LETTER_HAH => &["ha", "cha"],
        LETTER_KHAH => &["kho"],
        LETTER_DAL => &["dal"],
        LETTER_THAL => &["dzal", "dhal"],
        LETTER_REH => &["ro"],
        LETTER_ZAIN => &["za"],
        LETTER_SEEN => &["sin"],
        LETTER_SHEEN => &["syin", "shin"],
        LETTER_SAD => &["shod", "shot", "sod", "sot"],
        LETTER_DAD => &["dhod", "dhot", "dzod", "dzot", "dod", "dot"],
        LETTER_TAH => &["tho", "to"],
        LETTER_ZAH => &["dho", "dzo", "do"],
        LETTER_AIN => &["'ain", "ain"],
        LETTER_GHAIN => &["ghoin", "goin", "ghin", "gin"],
        LETTER_FEH => &["fa"],
        LETTER_QAF => &["qof", "kof"],
        LETTER_KAF => &["kaf"],
        LETTER_LAM => &["lam"],
        LETTER_MEEM => &["mim"],
        LETTER_NOON => &["nun"],
        LETTER_HEH => &["ha"],
        LETTER_WAW => &["wawu", "wau"],
        LETTER_YEH => &["ya"],
        _ => &[],
    }
}
