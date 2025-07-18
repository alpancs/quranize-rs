import { EventStatus } from "./event-status.js";
import init, { Quranize, compressExplanation } from "./engine/quranize.js";

let quranize;
let pendingMessage;

self.onmessage = message => {
    if (quranize === undefined) {
        pendingMessage = message;
        return;
    }
    const { data } = message;
    if (data.status === EventStatus.KeywordUpdated) {
        const { eventId, keyword } = data;
        const encodeResults = quranize.encode(keyword);
        self.postMessage({ status: EventStatus.KeywordEncoded, eventId, encodeResults });
    } else if (data.status === EventStatus.ResultClicked) {
        const { eventId, quran, expl } = data;
        const locations = quranize.getLocations(quran);
        const compactExpls = compressExplanation(quran, expl);
        self.postMessage({ status: EventStatus.ResultLocated, eventId, locations, compactExpls });
    }
};

await init({});
quranize = new Quranize();
self.postMessage({ status: EventStatus.WorkerInitiated });

if (pendingMessage) self.onmessage(pendingMessage);
