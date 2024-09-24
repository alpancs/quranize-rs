import EventStatus from "./event-status.js";
import init, { Quranize, compressExplanation } from "./quranize/quranize.js";

let quranize;
let pendingMessage;

self.onmessage = message => {
    if (quranize === undefined) {
        pendingMessage = message;
        return;
    }
    const data = message.data;
    switch (data.status) {
        case EventStatus.KeywordUpdated:
            const keyword = data.keyword;
            const encodeResults = quranize.encode(keyword);
            self.postMessage({ status: EventStatus.KeywordEncoded, keyword, encodeResults });
            break;
        case EventStatus.ResultClicked:
            const quran = data.quran;
            const compactExpls = compressExplanation(quran, data.expl);
            const locations = quranize.getLocations(quran);
            self.postMessage({ status: EventStatus.ResultLocated, quran, compactExpls, locations });
            break;
    }
};

self.postMessage({ status: EventStatus.EngineInitiationStarted });
await init({});
quranize = new Quranize();
self.postMessage({ status: EventStatus.EngineInitiated });

if (pendingMessage) self.onmessage(pendingMessage);
