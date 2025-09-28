export type EncodeResult = {
  quran: string;
  explanation: string;
  location_count: number;
};

export type SearchResult = {
  page: number;
  sura: number;
  aya: number;
  before_text: string;
  text: string;
  after_text: string;
};

export type Explanation = {
  alphabet: string;
  quran: string;
};
