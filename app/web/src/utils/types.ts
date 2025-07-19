export type EncodeResult = {
    quran: string;
    explanation: string;
    location_count: number;
}

export type SearchResult = {
    index: number;
    sura_number: number;
    aya_number: number;
    before_text: string;
    text: string;
    after_text: string;
}

export type Explanation = {
    alphabet: string;
    quran: string;
}
