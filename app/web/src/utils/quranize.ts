import { ref } from 'vue';
import type { Quranize } from "../workers/quranize/quranize-wasm";

export const initiated = ref(false);

const worker = new Worker(new URL("../workers/quranize/web-worker.ts", import.meta.url), { type: "module" });
const resolves = new Map<number, Function>();

worker.onmessage = ({ data: { id, resp } }) => {
    if (id === 0) {
        initiated.value = true
    } else {
        resolves.get(id)?.(resp)
        resolves.delete(id)
    }
};

let counter = 0;
export function call<T>(func: keyof Quranize, ...args: any[]) {
    const id = ++counter;
    const promise = new Promise<T>((resolve) => resolves.set(id, resolve));
    worker.postMessage({ id, func, args });
    return promise;
}
