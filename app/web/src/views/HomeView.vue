<script setup lang="ts">
import { ref, watch } from "vue";
import { initiated, call } from "../utils/quranize";
import type { EncodeResult } from "../utils/types";
import SearchBar from "../components/SearchBar.vue";
import SearchResult from "../components/SearchResult.vue";

const keyword = ref("");
const results = ref<EncodeResult[]>([]);

watch(keyword, async (text) => (results.value = await call("encode", text)));
</script>

<template>
    <div class="block">
        <SearchBar v-model="keyword" />
    </div>
    <div class="skeleton-block" v-if="!initiated && keyword"></div>
    <SearchResult :result v-for="result in results" />
</template>
