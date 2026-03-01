<script setup lang="ts">
import { ref } from "vue";
import { useRoute } from "vue-router";
import SearchResultDetail from "../components/SearchResultDetail.vue";
import { decode } from "../utils/chars";
import { call, initiated } from "../utils/quranize";
import type { SearchResult } from "../utils/types";

const route = useRoute();
const quran = decode(route.query.code as string);
const results = ref<SearchResult[]>([]);

call<SearchResult[]>("getLocations", quran).then((res) => results.value = res);
</script>

<template>
    <div class="block">
        <p class="quran-text has-text-centered has-text-weight-semibold is-size-4-touch is-size-3-desktop">
            {{ quran }}
        </p>
    </div>

    <div class="skeleton-block" v-if="!initiated"></div>
    <SearchResultDetail v-for="result in results"
        :key="`${result.sura}:${result.aya}:${result.spans[0]?.text}:${result.spans[1]?.text}:${result.spans[2]?.text}`"
        :result />
</template>
