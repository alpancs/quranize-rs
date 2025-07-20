import { ref } from 'vue';
import type { EncodeResult } from './types';

type Subject = 'encode' | 'search' | 'explain';

export function useQuranize() {
    const initiated = ref(false);
    const worker = new Worker("/src/workers/quranize/web-worker", { type: "module" });
    const resolves = new Map<number, (value: any) => void>();
    let counter = 0;

    function postToWorker<T>(subject: Subject, body: any) {
        const id = ++counter;
        const promise = new Promise<T>((resolve) => resolves.set(id, resolve));
        worker.postMessage({ id, subject, body });
        return promise;
    }

    const encode = (text: string) => postToWorker<EncodeResult[]>('encode', { text });
    const search = (quran: string) => postToWorker('search', { quran });
    const explain = (quran: string, expl: string) => postToWorker('explain', { quran, expl });

    worker.onmessage = ({ data: { id, response } }) => {
        if (id === 0) {
            initiated.value = true
        } else {
            resolves.get(id)?.(response)
            resolves.delete(id)
        }
    };

    return { initiated, encode, search, explain };
}
