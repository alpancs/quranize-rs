<script setup lang="ts">
import { ref, inject } from 'vue'

interface SearchResult {
    quran: string
    explanation: string
    location_count: number
}

const searchResults = ref<SearchResult[]>([])
const quranizeWorker = inject<Worker>('quranizeWorker')
let lastEventId = 0

quranizeWorker?.addEventListener('message', ({ data }) => {
    if (data.status === 'KeywordEncoded' && data.eventId > lastEventId) {
        lastEventId = data.eventId
        searchResults.value = data.encodeResults
    }
})
</script>

<template>
    <RouterLink v-for="result in searchResults"
        :to="{ path: '/result', query: { q: result.quran, e: result.explanation } }" class="box" dir="rtl">
        <p class="quran-text">{{ result.quran }}</p>
    </RouterLink>
</template>
