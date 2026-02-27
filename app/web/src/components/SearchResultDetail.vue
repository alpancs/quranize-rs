<script setup lang="ts">
import { getSuraNameAR } from "../utils/quranize";
import type { SearchResult } from "../utils/types";
import MarkedQuranText from "../components/MarkedQuranText.vue";
import AyaNumber from "../components/AyaNumber.vue";
defineProps<{ result: SearchResult }>();
</script>

<template>
    <RouterLink class="box"
        :to="{ name: 'QuranPage', params: { page: result.page }, query: { markedSura: result.sura, markedAya: result.aya } }">
        <p class="quran-text quran-paragraph is-size-5-touch is-size-4-desktop">
            <MarkedQuranText :beforeMarked="result.before_text" :marked="result.text"
                :afterMarked="result.after_text" />
            <AyaNumber class="mx-2" :aya="result.aya" />
            <span class="tag is-medium">
                <span class="has-text-weight-bold">
                    {{ getSuraNameAR(result.sura) }}
                </span>
            </span>
        </p>
    </RouterLink>
</template>
