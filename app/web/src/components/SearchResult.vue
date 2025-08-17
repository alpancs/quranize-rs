<script setup lang="ts">
import { inject, ref } from 'vue';
import { call, getSuraNameAR, toArabicNumber } from '../utils/quranize';
import type { SearchResult } from '../utils/types';
import MarkedQuranText from '../components/MarkedQuranText.vue';
import AyaNumber from '../components/AyaNumber.vue';

type PageItem = {
    sura: number;
    aya: number;
    text: string;
};

const { result } = defineProps<{ result: SearchResult }>();

const getTextID = inject<Function>('getTextID');
const isTranslationVisible = ref(false);
const isQuranPageVisible = ref(false);
const quranPageGroups = ref<PageItem[][]>([]);

function toggleTranslationVisibility() {
    isTranslationVisible.value = !isTranslationVisible.value;
}

function openQuranPage() {
    isQuranPageVisible.value = true;
    if (quranPageGroups.value.length === 0) updateQuranPageGroups();
}

async function updateQuranPageGroups() {
    const pageItems = await call<PageItem[]>('getPage', result.page);
    quranPageGroups.value = pageItems.reduce<PageItem[][]>((acc, curr, i, arr) => {
        if (i === 0 || arr[i - 1].sura !== curr.sura) acc.push([curr]);
        else acc[acc.length - 1].push(curr);
        return acc;
    }, []);
}

function closeQuranPage() {
    isQuranPageVisible.value = false;
}
</script>

<template>
    <div class="card">
        <header class="card-header" dir="rtl">
            <p class="card-header-title quran-text">
                <span class="icon-text">
                    <button class="icon" @click="openQuranPage">
                        <font-awesome-icon icon="fa-solid fa-book-open" />
                    </button>
                    {{ getSuraNameAR(result.sura) }} : {{ toArabicNumber(result.aya) }}
                </span>
            </p>
            <button class="card-header-icon" @click="toggleTranslationVisibility">
                <span class="icon">
                    <font-awesome-icon :icon="['fas', isTranslationVisible ? 'angle-up' : 'angle-down']" />
                </span>
            </button>
        </header>
        <div class="card-content">
            <div class="content">
                <p dir="rtl">
                    <MarkedQuranText :beforeMarked="result.before_text" :marked="result.text"
                        :afterMarked="result.after_text" @click="toggleTranslationVisibility" class="is-clickable" />
                </p>
                <p v-if="isTranslationVisible">{{ getTextID?.(result.sura, result.aya) }}</p>
            </div>
        </div>
    </div>

    <div class="modal is-active" v-if="isQuranPageVisible">
        <div class="modal-background" @click="closeQuranPage"></div>
        <div class="modal-card" dir="rtl">
            <header class="modal-card-head quran-text">
                <p class="modal-card-title">
                    {{ toArabicNumber(result.page) }}
                </p>
                <button class="delete" aria-label="close" @click="closeQuranPage"></button>
            </header>
            <section class="modal-card-body">
                <div class="quran-text" v-for="items in quranPageGroups">
                    <p class="has-text-centered" v-if="items[0].aya === 1">سورة {{ getSuraNameAR(items[0].sura) }}</p>
                    <p class="has-text-justified">
                        <span v-for="item in items">
                            <MarkedQuranText v-if="item.sura === result.sura && item.aya === result.aya"
                                :beforeMarked="result.before_text" :marked="result.text"
                                :afterMarked="result.after_text" />
                            <span v-else>{{ item.text }}</span>
                            <AyaNumber :aya="item.aya" />
                        </span>
                    </p>
                </div>
            </section>
            <footer class="modal-card-foot">
                <div class="buttons">
                    <router-link :to="{ path: '/quran-page', query: result }" class="button is-rounded">
                        <span class="icon">
                            <font-awesome-icon icon="fa-solid fa-up-right-and-down-left-from-center" />
                        </span>
                    </router-link>
                </div>
            </footer>
        </div>
    </div>
</template>
