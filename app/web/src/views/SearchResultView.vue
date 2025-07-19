<script setup lang="ts">
import { ref, inject } from 'vue';
import { useRoute } from 'vue-router';
import type { SearchResult, Explanation } from '../utils/types';

const searchResults = ref<SearchResult[]>([]);
const compactExpls = ref<Explanation[]>([]);
const route = useRoute();
const search = inject<(quran: string) => Promise<SearchResult[]>>('quranize.search');
const explain = inject<(quran: string, expl: string) => Promise<Explanation[]>>('quranize.explain');

const { q, e } = route.query;
const quran = (Array.isArray(q) ? q[0] : q) ?? '';
const expl = (Array.isArray(e) ? e[0] : e) ?? '';
search?.(quran).then((v) => searchResults.value = v);
explain?.(quran, expl).then((v) => compactExpls.value = v);

function toArabicNumber(n: number): string {
    if (n < 0) return `-${toArabicNumber(-n)}`;
    if (n < 10) return String.fromCharCode(0x0660 + n);
    return toArabicNumber(Math.trunc(n / 10)) + toArabicNumber(n % 10);
}

const SuraNames = ["الفاتحة", "البقرة", "آل عمران", "النساء", "المائدة", "الأنعام", "الأعراف", "الأنفال", "التوبة", "يونس", "هود", "يوسف", "الرعد", "ابراهيم", "الحجر", "النحل", "الإسراء", "الكهف", "مريم", "طه", "الأنبياء", "الحج", "المؤمنون", "النور", "الفرقان", "الشعراء", "النمل", "القصص", "العنكبوت", "الروم", "لقمان", "السجدة", "الأحزاب", "سبإ", "فاطر", "يس", "الصافات", "ص", "الزمر", "غافر", "فصلت", "الشورى", "الزخرف", "الدخان", "الجاثية", "الأحقاف", "محمد", "الفتح", "الحجرات", "ق", "الذاريات", "الطور", "النجم", "القمر", "الرحمن", "الواقعة", "الحديد", "المجادلة", "الحشر", "الممتحنة", "الصف", "الجمعة", "المنافقون", "التغابن", "الطلاق", "التحريم", "الملك", "القلم", "الحاقة", "المعارج", "نوح", "الجن", "المزمل", "المدثر", "القيامة", "الانسان", "المرسلات", "النبإ", "النازعات", "عبس", "التكوير", "الإنفطار", "المطففين", "الإنشقاق", "البروج", "الطارق", "الأعلى", "الغاشية", "الفجر", "البلد", "الشمس", "الليل", "الضحى", "الشرح", "التين", "العلق", "القدر", "البينة", "الزلزلة", "العاديات", "القارعة", "التكاثر", "العصر", "الهمزة", "الفيل", "قريش", "الماعون", "الكوثر", "الكافرون", "النصر", "المسد", "الإخلاص", "الفلق", "الناس"];
</script>

<template>
    <div class="block">
        <p class="quran-text title is-5 has-text-centered">{{ q }}</p>
        <div class="field is-grouped is-grouped-multiline is-justify-content-center">
            <div class="control" v-for="e in compactExpls">
                <div class="tags has-addons">
                    <span class="tag is-info">{{ e.alphabet }}</span>
                    <span class="tag"><span class="quran-text">{{ e.quran }}</span></span>
                </div>
            </div>
        </div>
    </div>

    <div class="card" dir="rtl" v-for="result in searchResults">
        <header class="card-header">
            <p class="card-header-title">
                <span class="quran-text">
                    {{ SuraNames[result.sura_number - 1] }} : {{ toArabicNumber(result.aya_number) }}
                </span>
            </p>
        </header>
        <div class="card-content">
            <div class="content">
                <p class="quran-text">
                    <span>{{ result.before_text }}</span>
                    <mark>{{ result.text }}</mark>
                    <span>{{ result.after_text }} &#xFD3F;{{ toArabicNumber(result.aya_number) }}&#xFD3E;</span>
                </p>
            </div>
        </div>
    </div>
</template>

<style scoped>
.card-header {
    border-bottom-style: groove;
    border-bottom-width: thin;
}
</style>
