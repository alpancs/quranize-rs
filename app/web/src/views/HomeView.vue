<script setup lang="ts">
import { ref, inject, watch, type Ref } from 'vue';
import SearchBar from '../components/SearchBar.vue';
import EncodeResult from '../components/EncodeResult.vue';
import type { EncodeResult as ER } from '../utils/types';

const initiated = inject<Ref<boolean>>('quranize.initiated');
const encode = inject<(text: string) => Promise<ER[]>>('quranize.encode');

const keyword = ref('');
const encodeResults = ref<ER[] | undefined>([]);

watch(keyword, async (newValue) => encodeResults.value = await encode?.(newValue));
</script>

<template>
    <div class="block">
        <SearchBar v-model="keyword" />
    </div>
    <div class="skeleton-block" v-if="!initiated && keyword"></div>
    <EncodeResult :result v-for="result in encodeResults" />
</template>
