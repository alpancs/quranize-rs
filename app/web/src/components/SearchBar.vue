<script setup lang="ts">
import { ref, inject, watch } from 'vue'

const workerInitiated = ref(false)
const keyword = ref('')
const placeholder = 'masyaallah'

const quranizeWorker = inject<Worker>('quranizeWorker')
watch(keyword, (newKeyword) => {
    quranizeWorker?.postMessage({ status: 'KeywordUpdated', keyword: newKeyword })
})
quranizeWorker?.addEventListener('message', ({ data: { status } }) => status === 'WorkerInitiated' && (workerInitiated.value = true))
</script>

<template>
    <div class="control has-icons-left" :class="{ 'is-loading': !workerInitiated }">
        <input class="input is-rounded" type="search" :placeholder="placeholder" spellcheck="false"
            v-model.trim="keyword" />
        <span class="icon is-left">🔍</span>
    </div>
</template>
