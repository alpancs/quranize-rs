import init, { Quranize } from "../utils/quranize-wasm";

interface Data {
  id: number;
  func: keyof Quranize;
  args: any[];
}

let quranize: Quranize | undefined;
let pendingEvents: MessageEvent<Data>[] | undefined = [];

const eventHandler = (event: MessageEvent<Data>) => {
  const {
    data: { id, func, args },
  } = event;
  if (quranize === undefined) pendingEvents?.push(event);
  else self.postMessage({ id, resp: (quranize[func] as Function)(...args) });
};
self.onmessage = eventHandler;

await init({});
quranize = new Quranize();
self.postMessage({ id: 0 });

pendingEvents.forEach(eventHandler);
pendingEvents = undefined;
