<script setup lang="ts">
import { ref } from "vue";
import { useRoute } from "vue-router";
import { initiated, call } from "../utils/quranize";
import type { SearchResult } from "../utils/types";
import SearchResultDetail from "../components/SearchResultDetail.vue";

const route = useRoute();
const results = ref<SearchResult[]>([]);
call<SearchResult[]>("getLocations", route.query.quran).then((res) => (results.value = res));
</script>

<template>
    <div class="block">
        <p class="quran-text has-text-centered has-text-weight-semibold is-size-4-touch is-size-3-desktop">
            {{ $route.query.quran }}
        </p>
    </div>

    <div class="skeleton-block" v-if="!initiated"></div>
    <SearchResultDetail :result v-for="result in results" />
</template>
