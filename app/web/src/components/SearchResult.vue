<script setup lang="ts">
import { inject, ref } from 'vue';
import { getSuraNameAR, toArabicNumber } from '../utils/quranize';
import type { SearchResult } from '../utils/types';
import MarkedQuranText from '../components/MarkedQuranText.vue';

defineProps<{ result: SearchResult }>();

const expanded = ref(false);
const getTextID = inject<Function>('getTextID');

function toggleExpanded() {
    expanded.value = !expanded.value;
}
</script>

<template>
    <div class="card">
        <header class="card-header" dir="rtl">
            <p class="card-header-title quran-text">
                <RouterLink class="icon-text" :to="{ path: '/quran-page', query: result }">
                    <span class="icon">
                        <font-awesome-icon icon="fa-solid fa-book" />
                    </span>
                    {{ getSuraNameAR(result.sura) }} : {{ toArabicNumber(result.aya) }}
                </RouterLink>
            </p>
            <button class="card-header-icon" @click="toggleExpanded">
                <span class="icon">
                    <font-awesome-icon :icon="['fas', expanded ? 'angle-up' : 'angle-down']" />
                </span>
            </button>
        </header>
        <div class="card-content">
            <div class="content">
                <p dir="rtl">
                    <MarkedQuranText :beforeMarked="result.before_text" :marked="result.text"
                        :afterMarked="result.after_text" @click="toggleExpanded" class="is-clickable" />
                </p>
                <p v-if="expanded">{{ getTextID?.(result.sura, result.aya) }}</p>
            </div>
        </div>
    </div>
</template>
