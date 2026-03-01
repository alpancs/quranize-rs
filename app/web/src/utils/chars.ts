const ArabicUnicodeMapping: Record<string, string> = {
  '\u{0020}': 'A', // SPACE
  '\u{0621}': 'B', // LETTER_HAMZA
  '\u{0622}': 'C', // LETTER_ALEF_WITH_MADDA_ABOVE
  '\u{0623}': 'D', // LETTER_ALEF_WITH_HAMZA_ABOVE
  '\u{0624}': 'E', // LETTER_WAW_WITH_HAMZA_ABOVE
  '\u{0625}': 'F', // LETTER_ALEF_WITH_HAMZA_BELOW
  '\u{0626}': 'G', // LETTER_YEH_WITH_HAMZA_ABOVE
  '\u{0627}': 'H', // LETTER_ALEF
  '\u{0628}': 'I', // LETTER_BEH
  '\u{0629}': 'J', // LETTER_TEH_MARBUTA
  '\u{062A}': 'K', // LETTER_TEH
  '\u{062B}': 'L', // LETTER_THEH
  '\u{062C}': 'M', // LETTER_JEEM
  '\u{062D}': 'N', // LETTER_HAH
  '\u{062E}': 'O', // LETTER_KHAH
  '\u{062F}': 'P', // LETTER_DAL
  '\u{0630}': 'Q', // LETTER_THAL
  '\u{0631}': 'R', // LETTER_REH
  '\u{0632}': 'S', // LETTER_ZAIN
  '\u{0633}': 'T', // LETTER_SEEN
  '\u{0634}': 'U', // LETTER_SHEEN
  '\u{0635}': 'V', // LETTER_SAD
  '\u{0636}': 'W', // LETTER_DAD
  '\u{0637}': 'X', // LETTER_TAH
  '\u{0638}': 'Y', // LETTER_ZAH
  '\u{0639}': 'Z', // LETTER_AIN
  '\u{063A}': 'a', // LETTER_GHAIN
  '\u{0640}': 'b', // TATWEEL
  '\u{0641}': 'c', // LETTER_FEH
  '\u{0642}': 'd', // LETTER_QAF
  '\u{0643}': 'e', // LETTER_KAF
  '\u{0644}': 'f', // LETTER_LAM
  '\u{0645}': 'g', // LETTER_MEEM
  '\u{0646}': 'h', // LETTER_NOON
  '\u{0647}': 'i', // LETTER_HEH
  '\u{0648}': 'j', // LETTER_WAW
  '\u{0649}': 'k', // LETTER_ALEF_MAKSURA
  '\u{064A}': 'l', // LETTER_YEH
  '\u{064B}': 'm', // FATHATAN
  '\u{064C}': 'n', // DAMMATAN
  '\u{064D}': 'o', // KASRATAN
  '\u{064E}': 'p', // FATHA
  '\u{064F}': 'q', // DAMMA
  '\u{0650}': 'r', // KASRA
  '\u{0651}': 's', // SHADDA
  '\u{0654}': 't', // HAMZA_ABOVE
  '\u{0670}': 'u', // LETTER_SUPERSCRIPT_ALEF
  '\u{06EA}': 'v'  // EMPTY_CENTRE_LOW_STOP
};

/**
 * Encodes a string of Arabic Unicode characters into
 * their corresponding A-Z/a-z mapping.
 */
export function encode(str: string): string {
  return Array.from(str)
    .map((char) => ArabicUnicodeMapping[char] || char)
    .join('');
}

const ReverseArabicMapping: Record<string, string> = Object.fromEntries(
  Object.entries(ArabicUnicodeMapping).map(([unicode, mappedChar]) => [mappedChar, unicode])
);

/**
 * Decodes a string of A-Z/a-z characters back into their
 * original Arabic Unicode characters.
 */
export function decode(encodedStr: string): string {
  return Array.from(encodedStr)
    .map((char) => ReverseArabicMapping[char] || char)
    .join('');
}
