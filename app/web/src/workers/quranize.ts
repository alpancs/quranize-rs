import init, { Quranize } from "../utils/quranize-wasm";

const quranizeProm = init().then(() => new Quranize());

self.onmessage = (event: MessageEvent) => {
  const { id, func, args } = event.data;
  quranizeProm.then((q) =>
    self.postMessage({ id, resp: (q[func] as Function)(...args) }),
  );
};

quranizeProm.then(() => self.postMessage({ id: 0 }));
