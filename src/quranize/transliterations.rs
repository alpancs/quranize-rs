const SPACE: char = '\u{0020}';

const LETTER_HAMZA: char = '\u{0621}';
const LETTER_ALEF_WITH_MADDA_ABOVE: char = '\u{0622}';
const LETTER_ALEF_WITH_HAMZA_ABOVE: char = '\u{0623}';
const LETTER_WAW_WITH_HAMZA_ABOVE: char = '\u{0624}';
const LETTER_ALEF_WITH_HAMZA_BELOW: char = '\u{0625}';
const LETTER_YEH_WITH_HAMZA_ABOVE: char = '\u{0626}';
const LETTER_ALEF: char = '\u{0627}';
const LETTER_BEH: char = '\u{0628}';
const LETTER_TEH_MARBUTA: char = '\u{0629}';
const LETTER_TEH: char = '\u{062A}';
const LETTER_THEH: char = '\u{062B}';
const LETTER_JEEM: char = '\u{062C}';
const LETTER_HAH: char = '\u{062D}';
const LETTER_KHAH: char = '\u{062E}';
const LETTER_DAL: char = '\u{062F}';
const LETTER_THAL: char = '\u{0630}';
const LETTER_REH: char = '\u{0631}';
const LETTER_ZAIN: char = '\u{0632}';
const LETTER_SEEN: char = '\u{0633}';
const LETTER_SHEEN: char = '\u{0634}';
const LETTER_SAD: char = '\u{0635}';
const LETTER_DAD: char = '\u{0636}';
const LETTER_TAH: char = '\u{0637}';
const LETTER_ZAH: char = '\u{0638}';
const LETTER_AIN: char = '\u{0639}';
const LETTER_GHAIN: char = '\u{063A}';

const TATWEEL: char = '\u{0640}';

const LETTER_FEH: char = '\u{0641}';
const LETTER_QAF: char = '\u{0642}';
const LETTER_KAF: char = '\u{0643}';
const LETTER_LAM: char = '\u{0644}';
const LETTER_MEEM: char = '\u{0645}';
const LETTER_NOON: char = '\u{0646}';
const LETTER_HEH: char = '\u{0647}';
const LETTER_WAW: char = '\u{0648}';
const LETTER_ALEF_MAKSURA: char = '\u{0649}';
const LETTER_YEH: char = '\u{064A}';

const FATHATAN: char = '\u{064B}';
const DAMMATAN: char = '\u{064C}';
const KASRATAN: char = '\u{064D}';
const FATHA: char = '\u{064E}';
const DAMMA: char = '\u{064F}';
const KASRA: char = '\u{0650}';
const SHADDA: char = '\u{0651}';

const HAMZA_ABOVE: char = '\u{0654}';
const LETTER_SUPERSCRIPT_ALEF: char = '\u{0670}';
const EMPTY_CENTRE_LOW_STOP: char = '\u{06EA}';

pub(super) fn map(c: char) -> &'static [&'static str] {
    match c {
        SPACE => &[""],

        LETTER_HAMZA => &["'", "a"],
        LETTER_ALEF_WITH_MADDA_ABOVE => &["a", "aa"],
        LETTER_ALEF_WITH_HAMZA_ABOVE => &["a", "u", "k", "'"],
        LETTER_WAW_WITH_HAMZA_ABOVE => &["u", "k", "'"],
        LETTER_ALEF_WITH_HAMZA_BELOW => &["i"],
        LETTER_YEH_WITH_HAMZA_ABOVE => &["'", "a", "i"],
        LETTER_ALEF => &["a", "aa", "o", "oo"],
        LETTER_BEH => &["b"],
        LETTER_TEH_MARBUTA => &["h", "t"],
        LETTER_TEH => &["t"],
        LETTER_THEH => &["ts", "s"],
        LETTER_JEEM => &["j"],
        LETTER_HAH => &["h", "kh", "ch"],
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
        SHADDA => &[],

        HAMZA_ABOVE => &["'", "a"],
        LETTER_SUPERSCRIPT_ALEF => &["a", "aa", "o", "oo"],

        _ => &[""],
    }
}

pub(super) fn contextual_map(prev_c: char, c: char) -> &'static [&'static str] {
    match (prev_c, c) {
        ('\0', LETTER_ALEF) => &["u", "i", ""],
        (LETTER_REH, EMPTY_CENTRE_LOW_STOP) => &["e"],
        (SPACE | LETTER_HAMZA | LETTER_WAW | FATHATAN | KASRA | HAMZA_ABOVE, LETTER_ALEF)
        | (LETTER_ALEF | KASRA, LETTER_LAM)
        | (LETTER_AIN, LETTER_WAW | LETTER_YEH | LETTER_SUPERSCRIPT_ALEF)
        | (LETTER_ALEF_MAKSURA, LETTER_SUPERSCRIPT_ALEF)
        | (FATHATAN..=SHADDA | EMPTY_CENTRE_LOW_STOP, LETTER_ALEF_MAKSURA)
        | (DAMMA, LETTER_WAW) => &[""],
        (_, SHADDA) => map(prev_c),
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
