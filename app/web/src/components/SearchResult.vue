<script setup lang="ts">
import type { SearchResult } from '../utils/types';

defineProps<{ result: SearchResult }>();

function toArabicNumber(n: number): string {
    if (n < 0) return `-${toArabicNumber(-n)}`;
    if (n < 10) return String.fromCharCode(0x0660 + n);
    return toArabicNumber(Math.trunc(n / 10)) + toArabicNumber(n % 10);
}

const SuraNames = ["الفاتحة", "البقرة", "آل عمران", "النساء", "المائدة", "الأنعام", "الأعراف", "الأنفال", "التوبة", "يونس", "هود", "يوسف", "الرعد", "ابراهيم", "الحجر", "النحل", "الإسراء", "الكهف", "مريم", "طه", "الأنبياء", "الحج", "المؤمنون", "النور", "الفرقان", "الشعراء", "النمل", "القصص", "العنكبوت", "الروم", "لقمان", "السجدة", "الأحزاب", "سبإ", "فاطر", "يس", "الصافات", "ص", "الزمر", "غافر", "فصلت", "الشورى", "الزخرف", "الدخان", "الجاثية", "الأحقاف", "محمد", "الفتح", "الحجرات", "ق", "الذاريات", "الطور", "النجم", "القمر", "الرحمن", "الواقعة", "الحديد", "المجادلة", "الحشر", "الممتحنة", "الصف", "الجمعة", "المنافقون", "التغابن", "الطلاق", "التحريم", "الملك", "القلم", "الحاقة", "المعارج", "نوح", "الجن", "المزمل", "المدثر", "القيامة", "الانسان", "المرسلات", "النبإ", "النازعات", "عبس", "التكوير", "الإنفطار", "المطففين", "الإنشقاق", "البروج", "الطارق", "الأعلى", "الغاشية", "الفجر", "البلد", "الشمس", "الليل", "الضحى", "الشرح", "التين", "العلق", "القدر", "البينة", "الزلزلة", "العاديات", "القارعة", "التكاثر", "العصر", "الهمزة", "الفيل", "قريش", "الماعون", "الكوثر", "الكافرون", "النصر", "المسد", "الإخلاص", "الفلق", "الناس"];
</script>

<template>
    <div class="card" dir="rtl">
        <header class="card-header">
            <p class="card-header-title">
                <RouterLink :to="{ path: '/quran-page', query: result }" class="quran-text">
                    {{ SuraNames[result.sura_number - 1] }} : {{ toArabicNumber(result.aya_number) }}
                </RouterLink>
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
.card {
    border-style: solid;
    border-width: 1px;
}

.card-header {
    border-bottom-style: dashed;
    border-bottom-width: 1px;
}
</style>
