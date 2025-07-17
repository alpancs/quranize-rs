<script setup lang="ts">
import { ref, inject } from 'vue'

interface SearchResult {
    quran: string
    explanation: string
    location_count: number
}

const searchResults = ref<SearchResult[]>([])

const quranizeWorker = inject<Worker>('quranizeWorker')
quranizeWorker?.addEventListener('message', ({ data }) => {
    if (data.status === 'KeywordEncoded') searchResults.value = data.encodeResults
})
</script>

<template>
    <div v-for="result in searchResults" class="box quran-text" dir="rtl">
        {{ result.quran }}
    </div>
</template>
