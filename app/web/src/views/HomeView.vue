<script setup lang="ts">
import { ref, inject, watch } from 'vue';
import SearchBar from '../components/SearchBar.vue';
import EncodeResult from '../components/EncodeResult.vue';
import type { EncodeResult as ER } from '../utils/types';

const encode = inject<(text: string) => Promise<ER[]>>('quranize.encode');

const keyword = ref('');
const encodeResults = ref<ER[] | undefined>([]);

watch(keyword, async (newValue) => encodeResults.value = await encode?.(newValue));
</script>

<template>
    <div class="block">
        <SearchBar v-model="keyword" />
    </div>
    <EncodeResult :result v-for="result in encodeResults" />
</template>
