<script setup lang="ts">
import { inject, ref, watch } from 'vue';
import SearchBar from '../components/SearchBar.vue'
import SearchResult from '../components/SearchResult.vue'

interface SearchResult {
    quran: string
    explanation: string
    location_count: number
}

const keyword = ref('')
const encode = inject<(text: string) => Promise<SearchResult[]>>('quranize.encode')
const searchResults = ref<SearchResult[]>([])
watch(keyword, async (newValue) => searchResults.value = await encode?.(newValue) ?? [])
</script>

<template>
    <div class="block">
        <SearchBar v-model="keyword" />
    </div>
    <SearchResult :searchResults />
</template>

<style scoped></style>
