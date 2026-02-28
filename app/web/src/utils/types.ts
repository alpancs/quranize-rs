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
  spans: Span[];
};

type Span = {
  text: string;
  marked: boolean;
};
