<script setup lang="ts">
import { ref, watch } from 'vue';
import { initiated, call } from '../utils/quranize';
import type { EncodeResult as ER } from '../utils/types';
import SearchBar from '../components/SearchBar.vue';
import EncodeResult from '../components/EncodeResult.vue';

const keyword = ref('');
const encodeResults = ref<ER[]>([]);

watch(keyword, async (text) => encodeResults.value = await call('encode', text));
</script>

<template>
    <div class="block">
        <SearchBar v-model="keyword" />
    </div>
    <div class="skeleton-block" v-if="!initiated && keyword"></div>
    <EncodeResult :result v-for="result in encodeResults" />
</template>
