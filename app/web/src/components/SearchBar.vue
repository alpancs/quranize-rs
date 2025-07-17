<script setup lang="ts">
import { ref, inject, watch } from 'vue'

const workerInitiated = ref(false)
const keyword = ref('')
const placeholder = 'masyaallah'
const quranizeWorker = inject<Worker>('quranizeWorker')
let eventId = 0

quranizeWorker?.addEventListener('message', ({ data: { status } }) => {
    if (status === 'WorkerInitiated') workerInitiated.value = true
})

watch(keyword, (newKeyword) => {
    const message = { status: 'KeywordUpdated', eventId: ++eventId, keyword: newKeyword }
    quranizeWorker?.postMessage(message)
})
</script>

<template>
    <div class="control has-icons-left" :class="{ 'is-loading': !workerInitiated }">
        <input class="input is-rounded" type="search" :placeholder="placeholder" spellcheck="false"
            v-model.trim="keyword" />
        <span class="icon is-left">🔍</span>
    </div>
</template>
