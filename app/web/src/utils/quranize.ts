import { ref } from 'vue'

interface SearchResult {
    quran: string
    explanation: string
    location_count: number
}

export function useQuranize() {
    let idOffset = 0
    const worker = new Worker("/src/workers/quranize/web-worker", { type: "module" })
    const initialized = getInitialized(worker)
    const encode = (text: string) => getSearchResultsProm(worker, ++idOffset, text)
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

function getSearchResultsProm(worker: Worker, eventId: number, text: string) {
    console.log(eventId, text)
    const searchResults = new Promise<SearchResult[]>((resolve) => {
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
