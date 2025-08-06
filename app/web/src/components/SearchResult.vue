<script setup lang="ts">
import { getSuraName, toArabicNumber } from '../utils/quranize';
import type { SearchResult } from '../utils/types';
import MarkedQuranText from '../components/MarkedQuranText.vue';

defineProps<{ result: SearchResult }>();
</script>

<template>
    <article class="message" dir="rtl">
        <div class="message-body">
            <span class="quran-text pl-2">
                <RouterLink :to="{ path: '/quran-page', query: result }" class="button">
                    {{ getSuraName(result.sura) }} : {{ toArabicNumber(result.aya) }}
                </RouterLink>
            </span>
            <MarkedQuranText :beforeMarked="result.before_text" :marked="result.text"
                :afterMarked="result.after_text" />
        </div>
    </article>
</template>
