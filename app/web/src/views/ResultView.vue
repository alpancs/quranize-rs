<script setup lang="ts">
import { ref, inject } from 'vue'
import { useRoute } from 'vue-router'

interface SearchResult {
    index: number
    sura_number: number
    aya_number: number
    before_text: string
    text: string
    after_text: string
}

const searchResults = ref<SearchResult[]>([])
const compactExpls = ref<any[]>([])
const route = useRoute()
const { q, e } = route.query
const quranizeWorker = inject<Worker>('quranizeWorker')
let eventId = 0

const message = { status: 'ResultClicked', eventId: ++eventId, quran: q, expl: e }
quranizeWorker?.postMessage(message)

quranizeWorker?.addEventListener('message', ({ data }) => {
    if (data.status === 'ResultLocated' && data.eventId === eventId) {
        searchResults.value = data.locations
        compactExpls.value = data.compactExpls
    }
})
</script>

<template>
    <div class="block">
        <p class="quran-text title is-5 has-text-centered">{{ q }}</p>
        <div class="field is-grouped is-grouped-multiline is-justify-content-center">
            <div class="control" v-for="e in compactExpls">
                <div class="tags has-addons">
                    <span class="tag is-info">{{ e.alphabet }}</span>
                    <span class="tag"><span class="quran-text">{{ e.quran }}</span></span>
                </div>
            </div>
        </div>
    </div>

    <article class="message" v-for="result in searchResults">
        <div class="message-header">
            <p>sura {{ result.sura_number }}, aya {{ result.aya_number }}</p>
        </div>
        <div class="message-body">
            <p class="quran-text">
                {{ result.before_text }}<mark>{{ result.text }}</mark>{{ result.after_text }}
            </p>
        </div>
    </article>
</template>

<style scoped></style>
