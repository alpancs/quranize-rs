const ArabicUnicodeMapping: Record<string, string> = {
  '\u{000A}': 'A', // LINE_FEED
  '\u{0020}': 'B', // SPACE
  '\u{0621}': 'C', // ARABIC_LETTER_HAMZA
  '\u{0622}': 'D', // ARABIC_LETTER_ALEF_WITH_MADDA_ABOVE
  '\u{0623}': 'E', // ARABIC_LETTER_ALEF_WITH_HAMZA_ABOVE
  '\u{0624}': 'F', // ARABIC_LETTER_WAW_WITH_HAMZA_ABOVE
  '\u{0625}': 'G', // ARABIC_LETTER_ALEF_WITH_HAMZA_BELOW
  '\u{0626}': 'H', // ARABIC_LETTER_YEH_WITH_HAMZA_ABOVE
  '\u{0627}': 'I', // ARABIC_LETTER_ALEF
  '\u{0628}': 'J', // ARABIC_LETTER_BEH
  '\u{0629}': 'K', // ARABIC_LETTER_TEH_MARBUTA
  '\u{062A}': 'L', // ARABIC_LETTER_TEH
  '\u{062B}': 'M', // ARABIC_LETTER_THEH
  '\u{062C}': 'N', // ARABIC_LETTER_JEEM
  '\u{062D}': 'O', // ARABIC_LETTER_HAH
  '\u{062E}': 'P', // ARABIC_LETTER_KHAH
  '\u{062F}': 'Q', // ARABIC_LETTER_DAL
  '\u{0630}': 'R', // ARABIC_LETTER_THAL
  '\u{0631}': 'S', // ARABIC_LETTER_REH
  '\u{0632}': 'T', // ARABIC_LETTER_ZAIN
  '\u{0633}': 'U', // ARABIC_LETTER_SEEN
  '\u{0634}': 'V', // ARABIC_LETTER_SHEEN
  '\u{0635}': 'W', // ARABIC_LETTER_SAD
  '\u{0636}': 'X', // ARABIC_LETTER_DAD
  '\u{0637}': 'Y', // ARABIC_LETTER_TAH
  '\u{0638}': 'Z', // ARABIC_LETTER_ZAH
  '\u{0639}': 'a', // ARABIC_LETTER_AIN
  '\u{063A}': 'b', // ARABIC_LETTER_GHAIN
  '\u{0641}': 'c', // ARABIC_LETTER_FEH
  '\u{0642}': 'd', // ARABIC_LETTER_QAF
  '\u{0643}': 'e', // ARABIC_LETTER_KAF
  '\u{0644}': 'f', // ARABIC_LETTER_LAM
  '\u{0645}': 'g', // ARABIC_LETTER_MEEM
  '\u{0646}': 'h', // ARABIC_LETTER_NOON
  '\u{0647}': 'i', // ARABIC_LETTER_HEH
  '\u{0648}': 'j', // ARABIC_LETTER_WAW
  '\u{0649}': 'k', // ARABIC_LETTER_ALEF_MAKSURA
  '\u{064A}': 'l', // ARABIC_LETTER_YEH
  '\u{064B}': 'm', // ARABIC_FATHATAN
  '\u{064C}': 'n', // ARABIC_DAMMATAN
  '\u{064D}': 'o', // ARABIC_KASRATAN
  '\u{064E}': 'p', // ARABIC_FATHA
  '\u{064F}': 'q', // ARABIC_DAMMA
  '\u{0650}': 'r', // ARABIC_KASRA
  '\u{0651}': 's', // ARABIC_SHADDA
  '\u{0652}': 't', // ARABIC_SUKUN
  '\u{0670}': 'u', // ARABIC_LETTER_SUPERSCRIPT_ALEF
  '\u{06D6}': 'v', // ARABIC_SMALL_HIGH_LIGATURE_SAD_WITH_LAM_WITH_ALEF_MAKSURA
  '\u{06D7}': 'w', // ARABIC_SMALL_HIGH_LIGATURE_QAF_WITH_LAM_WITH_ALEF_MAKSURA
  '\u{06D8}': 'x', // ARABIC_SMALL_HIGH_MEEM_INITIAL_FORM
  '\u{06D9}': 'y', // ARABIC_SMALL_HIGH_LAM_ALEF
  '\u{06DA}': 'z', // ARABIC_SMALL_HIGH_JEEM
  '\u{06DB}': '0', // ARABIC_SMALL_HIGH_THREE_DOTS
  '\u{06DC}': '1', // ARABIC_SMALL_HIGH_SEEN
  '\u{06DE}': '2', // ARABIC_START_OF_RUB_EL_HIZB
  '\u{06E9}': '3', // ARABIC_PLACE_OF_SAJDAH
};

/**
 * Encodes a string of Arabic Unicode characters into
 * their corresponding A-Z/a-z mapping.
 */
export function encode(str: string): string {
  let result = '';
  for (let i = 0; i < str.length; i++) {
    result += ArabicUnicodeMapping[str[i]!] || str[i];
  }
  return result;
}

const ReverseArabicMapping: Record<string, string> = Object.fromEntries(
  Object.entries(ArabicUnicodeMapping).map(([unicode, mappedChar]) => [mappedChar, unicode])
);

/**
 * Decodes a string of A-Z/a-z characters back into their
 * original Arabic Unicode characters.
 */
export function decode(encodedStr: string): string {
  let result = '';
  for (let i = 0; i < encodedStr.length; i++) {
    result += ReverseArabicMapping[encodedStr[i]!] || encodedStr[i];
  }
  return result;
}
