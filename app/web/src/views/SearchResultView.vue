<script setup lang="ts">
import { ref } from "vue";
import { useRoute } from "vue-router";
import SearchResultDetail from "../components/SearchResultDetail.vue";
import { call, initiated } from "../utils/quranize";
import type { SearchResult } from "../utils/types";

function decode(str: string): string {
    const binString = atob(str);
    const bytes = Uint8Array.from(binString, (m) => m.codePointAt(0)!);
    return new TextDecoder().decode(bytes);
}

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
