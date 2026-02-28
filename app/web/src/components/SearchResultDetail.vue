<script setup lang="ts">
import AyaNumber from "../components/AyaNumber.vue";
import { getSuraNameAR } from "../utils/quranize";
import type { SearchResult } from "../utils/types";
defineProps<{ result: SearchResult }>();
</script>

<template>
    <RouterLink class="box"
        :to="{ name: 'QuranPage', params: { page: result.page }, query: { markedSura: result.sura, markedAya: result.aya } }">
        <p class="quran-text quran-paragraph is-size-5-touch is-size-4-desktop">
            <component v-for="span in result.spans" :is="span.marked ? 'mark' : 'span'">{{ span.text }}</component>
            <AyaNumber class="mx-2" :aya="result.aya" />
            <span class="tag is-medium">
                <span class="has-text-weight-bold">
                    {{ getSuraNameAR(result.sura) }}
                </span>
            </span>
        </p>
    </RouterLink>
</template>
