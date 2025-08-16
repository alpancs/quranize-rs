<script setup lang="ts">
import { getSuraNameAR, toArabicNumber } from '../utils/quranize';
import type { SearchResult } from '../utils/types';
import MarkedQuranText from '../components/MarkedQuranText.vue';

defineProps<{ result: SearchResult }>();
</script>

<template>
    <div class="card" dir="rtl">
        <header class="card-header">
            <p class="card-header-title quran-text">
                <RouterLink class="icon-text" :to="{ path: '/quran-page', query: result }">
                    <span class="icon">
                        <font-awesome-icon icon="fa-solid fa-book" />
                    </span>
                    {{ getSuraNameAR(result.sura) }} : {{ toArabicNumber(result.aya) }}
                </RouterLink>
            </p>
            <button class="card-header-icon">
                <span class="icon">
                    <font-awesome-icon icon="fa-solid fa-angle-down" />
                </span>
            </button>
        </header>
        <div class="card-content">
            <MarkedQuranText :beforeMarked="result.before_text" :marked="result.text"
                :afterMarked="result.after_text" />
        </div>
    </div>
</template>
