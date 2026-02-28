const textIDMap = (async function () {
  const map = new Map<string, string>();
  const plainTextID = (await import("./quran/id.indonesian.txt?raw")).default;
  plainTextID.split("\n").forEach((line) => {
    const split = line.split("|");
    if (split.length === 3) {
      const [sura, aya, text] = split;
      map.set(`${sura}/${aya}`, text!);
    }
  });
  return map;
})();

export const getTextID = async (sura: number, aya: number) => (await textIDMap).get(`${sura}/${aya}`);
