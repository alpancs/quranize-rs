<script setup lang="ts">
import { getSuraName, toArabicNumber } from '../utils/quranize';
import type { SearchResult } from '../utils/types';
import MarkedQuranText from '../components/MarkedQuranText.vue';
import AyaNumber from '../components/AyaNumber.vue';

defineProps<{ result: SearchResult }>();
</script>

<template>
    <div class="card" dir="rtl">
        <header class="card-header">
            <p class="card-header-title">
                <RouterLink :to="{ path: '/quran-page', query: result }" class="quran-text">
                    {{ getSuraName(result.sura) }} : {{ toArabicNumber(result.aya) }}
                </RouterLink>
            </p>
        </header>
        <div class="card-content">
            <div class="content">
                <MarkedQuranText :beforeMarked="result.before_text" :marked="result.text"
                    :afterMarked="result.after_text" />
                <AyaNumber :aya="result.aya" />
            </div>
        </div>
    </div>
</template>

<style scoped>
.card {
    border-style: solid;
    border-width: 1px;
}

.card-header {
    border-bottom-style: dashed;
    border-bottom-width: 1px;
}
</style>
