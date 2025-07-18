import { ref } from 'vue'
import type { SearchResults } from '../types/search-result'

export function useQuranize() {
    let idOffset = 0
    const worker = new Worker("/src/workers/quranize/web-worker", { type: "module" })
    const initialized = getInitialized(worker)
    const encode = (text: string) => getSearchResults(worker, ++idOffset, text)
    return { initialized, encode }
}

function getInitialized(worker: Worker) {
    const initialized = ref(false)
    const controller = new AbortController()
    worker.addEventListener('message', ({ data: { status } }) => {
        if (status === 'WorkerInitiated') {
            controller.abort()
            initialized.value = true
        }
    }, { signal: controller.signal })
    return initialized
}

function getSearchResults(worker: Worker, eventId: number, text: string) {
    console.log(eventId, text)
    const searchResults = new Promise<SearchResults>((resolve) => {
        const controller = new AbortController()
        worker.addEventListener('message', ({ data }) => {
            if (data.status === 'KeywordEncoded' && data.eventId === eventId) {
                controller.abort()
                resolve(data.encodeResults)
            }
        }, { signal: controller.signal })
    })
    worker.postMessage({ status: 'KeywordUpdated', eventId, keyword: text })
    return searchResults
}
