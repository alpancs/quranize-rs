<script setup lang="ts">
import { inject, ref } from 'vue';
import { getPageItemGroups, getSuraNameAR, toArabicNumber, type PageItem } from '../utils/quranize';
import type { SearchResult } from '../utils/types';
import MarkedQuranText from '../components/MarkedQuranText.vue';
import AyaNumber from '../components/AyaNumber.vue';

const { result } = defineProps<{ result: SearchResult }>();

const isTranslationVisible = ref(false);
const isQuranPageVisible = ref(false);
const pageItemGroups = ref<PageItem[][]>([]);
const textID = ref('');

inject<Function>('getTextID')?.(result.sura, result.aya)
    .then((v: string) => textID.value = v);

function toggleTranslationVisibility() {
    isTranslationVisible.value = !isTranslationVisible.value;
}

async function openQuranPage() {
    isQuranPageVisible.value = true;
    if (pageItemGroups.value.length === 0)
        pageItemGroups.value = await getPageItemGroups(result.page);
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
                <p v-if="isTranslationVisible">{{ textID }}</p>
            </div>
        </div>
    </div>

    <div class="modal is-active" v-if="isQuranPageVisible">
        <div class="modal-background" @click="closeQuranPage"></div>
        <div class="modal-card">
            <header class="modal-card-head mt-6">
                <p class="modal-card-title">
                    <router-link :to="{ path: '/quran-page', query: result }" class="tag is-medium">
                        <span class="icon">
                            <font-awesome-icon icon="fa-solid fa-up-right-and-down-left-from-center" />
                        </span>
                    </router-link>
                </p>
                <button class="delete" aria-label="close" @click="closeQuranPage"></button>
            </header>
            <section class="modal-card-body" dir="rtl">
                <div class="quran-text" v-for="items in pageItemGroups">
                    <p class="has-text-centered mt-4 has-text-weight-semibold" v-if="items[0].aya === 1">
                        سورة {{ getSuraNameAR(items[0].sura) }}
                    </p>
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
        </div>
    </div>
</template>
