<script setup lang="ts">
import { inject, ref, watch } from 'vue';
import SearchBar from '../components/SearchBar.vue'
import EncodeResult from '../components/EncodeResult.vue'
import type { SearchResults } from '../types/search-result'

const keyword = ref('')
const encode = inject<(text: string) => Promise<SearchResults>>('quranize.encode')
const searchResults = ref<SearchResults>([])
watch(keyword, async (newValue) => searchResults.value = await encode?.(newValue) ?? [])
</script>

<template>
    <div class="block">
        <SearchBar v-model="keyword" />
    </div>
    <EncodeResult :result v-for="result in searchResults" />
</template>

<style scoped></style>
