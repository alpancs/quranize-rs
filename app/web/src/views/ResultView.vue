<script setup lang="ts">
import { ref, inject } from 'vue'
import { useRoute } from 'vue-router'

interface SearchResult {
    index: number
    sura_number: number
    aya_number: number
    before_text: string
    text: string
    after_text: string
}

const searchResults = ref<SearchResult[]>([])
const compactExpls = ref<any[]>([])
const route = useRoute()
const { q, e } = route.query
const quranizeWorker = inject<Worker>('quranizeWorker')
let eventId = 0

function suraName(n: number): string {
    return SuraNames[n - 1]
}

function toArabicNumber(n: number): string {
    if (n < 0) return `-${toArabicNumber(-n)}`;
    if (n < 10) return String.fromCharCode(0x0660 + n);
    return toArabicNumber(Math.trunc(n / 10)) + toArabicNumber(n % 10);
}

const message = { status: 'ResultClicked', eventId: ++eventId, quran: q, expl: e }
quranizeWorker?.postMessage(message)

quranizeWorker?.addEventListener('message', ({ data }) => {
    if (data.status === 'ResultLocated' && data.eventId === eventId) {
        searchResults.value = data.locations
        compactExpls.value = data.compactExpls
    }
})

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
                    {{ suraName(result.sura_number) }} : {{ toArabicNumber(result.aya_number) }}
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
