<script setup lang="ts">
import { getSuraNameAR } from "../utils/quranize";
import type { SearchResult } from "../utils/types";
import MarkedQuranText from "../components/MarkedQuranText.vue";
import AyaNumber from "../components/AyaNumber.vue";

const { result } = defineProps<{ result: SearchResult }>();
const toQuranPage = {
    name: "QuranPage",
    params: { page: result.page },
    query: { markedSura: result.sura, markedAya: result.aya },
};
</script>

<template>
    <RouterLink :to="toQuranPage" class="box">
        <p class="quran-text quran-paragraph is-size-5">
            <span class="tag is-large ml-2">{{ getSuraNameAR(result.sura) }}</span>
            <MarkedQuranText :beforeMarked="result.before_text" :marked="result.text"
                :afterMarked="result.after_text" />
            <AyaNumber :aya="result.aya" />
        </p>
    </RouterLink>
</template>
