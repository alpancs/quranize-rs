import init, { Quranize, compressExplanation as explain } from "./quranize-wasm";

let quranize: Quranize | undefined;
let pendingEvents: MessageEvent<any>[] | undefined = [];

self.onmessage = (event) => {
    const { data: { id, subject, body } } = event;

    if (quranize === undefined)
        return pendingEvents?.push(event);

    if (subject === 'encode')
        return self.postMessage({ id, response: quranize.encode(body.text) });

    if (subject === 'search')
        return self.postMessage({ id, response: quranize.getLocations(body.quran) });

    if (subject === 'explain')
        return self.postMessage({ id, response: explain(body.quran, body.expl) });

    if (subject === 'getQuran')
        return self.postMessage({ id, response: quranize.getQuran(body.index) });
};

await init({});
quranize = new Quranize();
self.postMessage({ id: 0 });

pendingEvents.forEach(self.onmessage);
pendingEvents = undefined;
