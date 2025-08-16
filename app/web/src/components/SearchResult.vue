<script setup lang="ts">
import { inject, ref } from 'vue';
import { useRouter } from 'vue-router';
import { getSuraNameAR, toArabicNumber } from '../utils/quranize';
import type { SearchResult } from '../utils/types';
import MarkedQuranText from '../components/MarkedQuranText.vue';

const { result } = defineProps<{ result: SearchResult }>();
const router = useRouter();

const expanded = ref(false);
const getTextID = inject<Function>('getTextID');

function toggleExpanded() {
    expanded.value = !expanded.value;
}

function toQuranPage() {
    router.push({ path: '/quran-page', query: result });
}
</script>

<template>
    <div class="card">
        <header class="card-header" dir="rtl">
            <p class="card-header-title quran-text">
                <span class="icon-text is-clickable" @click="toQuranPage">
                    <span class="icon">
                        <font-awesome-icon icon="fa-solid fa-book-open" />
                    </span>
                    {{ getSuraNameAR(result.sura) }} : {{ toArabicNumber(result.aya) }}
                </span>
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
