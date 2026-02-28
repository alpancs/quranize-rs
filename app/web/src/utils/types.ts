export type EncodeResult = {
  quran: string;
  location_count: number;
  explanations: Explanation[];
};

export type Explanation = {
  alphabet: string;
  quran: string;
};

export type SearchResult = {
  page: number;
  sura: number;
  aya: number;
  spans: SearchResultSpan[];
};

type SearchResultSpan = {
  text: string;
  marked: boolean;
};
