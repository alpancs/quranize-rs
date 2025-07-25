<script setup lang="ts">
import { ref } from 'vue';
import { useRoute } from 'vue-router';
import { call, toArabicNumber } from '../utils/quranize';
import MarkedQuranText from '../components/MarkedQuranText.vue';
import AyaNumber from '../components/AyaNumber.vue';

type QuranPageData = {
    sura: number;
    aya: number;
    text: string;
}[];

const route = useRoute();
const page = parseInt(route.query.page as string);
const sura = parseInt(route.query.sura as string);
const aya = parseInt(route.query.aya as string);
const beforeText = route.query.before_text as string;
const text = route.query.text as string;
const afterText = route.query.after_text as string;

const quranData = ref<QuranPageData>([]);

call<QuranPageData>('getPage', page).then((v) => quranData.value = v);
</script>

<template>
    <div class="block">
        <div class="has-text-justified is-size-5" dir="rtl">
            <span v-for="(quran, index) in quranData" :key="index">
                <MarkedQuranText :beforeMarked="beforeText" :marked="text" :afterMarked="afterText"
                    v-if="quran.sura === sura && quran.aya === aya" />
                <span class="quran-text" v-else>{{ quran.text }}</span>
                <AyaNumber :aya="quran.aya" />
            </span>
        </div>
    </div>
    <div class="block">
        <p class="quran-text subtitle has-text-centered is-size-5">◄ {{ toArabicNumber(page) }} ►</p>
    </div>
</template>
