import init, { Quranize } from "../utils/quranize-wasm";

const quranizeProm = init().then(() => new Quranize());

self.onmessage = (event: MessageEvent) => {
  const { id, func, args } = event.data;
  quranizeProm.then((q) => {
    let resp: unknown;
    switch (func) {
      case "encode":
        resp = q.encode(...args);
        break;
      case "getLocations":
        resp = q.getLocations(...args);
        break;
      case "getPage":
        resp = q.getPage(...args);
        break;
      default:
        resp = undefined;
    }
    self.postMessage({ id, resp });
  });
};

quranizeProm.then(() => self.postMessage({ id: 0 }));
