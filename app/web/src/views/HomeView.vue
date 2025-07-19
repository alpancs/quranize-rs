<script setup lang="ts">
import { inject, ref, watch } from 'vue';
import SearchBar from '../components/SearchBar.vue'
import EncodeResult from '../components/EncodeResult.vue'
import type { EncodeResult as ER } from '../utils/types'

const keyword = ref('')
const encode = inject<(text: string) => Promise<ER[]>>('quranize.encode')
const searchResults = ref<ER[]>([])
watch(keyword, async (newValue) => searchResults.value = await encode?.(newValue) ?? [])
</script>

<template>
    <div class="block">
        <SearchBar v-model="keyword" />
    </div>
    <EncodeResult :result v-for="result in searchResults" />
</template>

<style scoped></style>
