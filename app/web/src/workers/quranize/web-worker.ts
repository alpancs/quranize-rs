import init, { Quranize, compressExplanation as explain } from "./quranize-wasm";

interface Data {
    id: number;
    func: keyof Quranize | 'explain';
    args: any[];
}

let quranize: Quranize | undefined;
let pendingEvents: MessageEvent<Data>[] | undefined = [];

const eventHandler = (event: MessageEvent<Data>) => {
    const { data: { id, func, args } } = event;
    if (quranize === undefined) return pendingEvents?.push(event);
    if (func === 'explain') return self.postMessage({ id, resp: explain(args[0], args[1]) });
    self.postMessage({ id, resp: (quranize[func] as Function)(...args) });
};
self.onmessage = eventHandler;

await init({});
quranize = new Quranize();
self.postMessage({ id: 0 });

pendingEvents.forEach(eventHandler);
pendingEvents = undefined;
