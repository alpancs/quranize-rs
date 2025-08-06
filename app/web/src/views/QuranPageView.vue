<script setup lang="ts">
import { ref, watch } from 'vue';
import { useRoute } from 'vue-router';
import { call, toArabicNumber, getSuraName } from '../utils/quranize';
import MarkedQuranText from '../components/MarkedQuranText.vue';
import AyaNumber from '../components/AyaNumber.vue';

type PageItem = {
    sura: number;
    aya: number;
    text: string;
};

const route = useRoute();
const page = ref(parseInt(route.query.page as string));
const sura = parseInt(route.query.sura as string);
const aya = parseInt(route.query.aya as string);
const beforeText = route.query.before_text as string;
const text = route.query.text as string;
const afterText = route.query.after_text as string;

const pageItems = ref<PageItem[]>([]);

call<PageItem[]>('getPage', page.value).then((v) => pageItems.value = v);

watch(() => route.query.page, async (newPage) => {
    page.value = parseInt(newPage as string);
    pageItems.value = await call<PageItem[]>('getPage', page.value);
});
</script>

<template>
    <div class="box">
        <div class="has-text-justified" dir="rtl">
            <span v-for="item in pageItems">
                <p v-if="item.aya === 1" class="mt-4 quran-text has-text-centered has-text-weight-bold">
                    سورة {{ getSuraName(item.sura) }}
                </p>
                <MarkedQuranText :beforeMarked="beforeText" :marked="text" :afterMarked="afterText"
                    v-if="item.sura === sura && item.aya === aya" />
                <span class="quran-text" v-else>{{ item.text }}</span>
                <AyaNumber :aya="item.aya" />
            </span>
        </div>
    </div>
    <div class="buttons has-addons is-centered quran-text are-small">
        <RouterLink class="button is-rounded" v-if="page < 604"
            :to="{ query: { page: page + 1, sura, aya, before_text: beforeText, text, after_text: afterText } }">
            <span class="icon"><font-awesome-icon icon="fa-solid fa-caret-left" /></span>
            <span>{{ toArabicNumber(page + 1) }}</span>
        </RouterLink>
        <span v-else class="button is-rounded" disabled>
            <span class="icon"><font-awesome-icon icon="fa-solid fa-caret-left" /></span>
        </span>

        <span class="button is-info">{{ toArabicNumber(page) }}</span>

        <RouterLink class="button is-rounded" v-if="page > 1"
            :to="{ query: { page: page - 1, sura, aya, before_text: beforeText, text, after_text: afterText } }">
            <span>{{ toArabicNumber(page - 1) }}</span>
            <span class="icon"><font-awesome-icon icon="fa-solid fa-caret-right" /></span>
        </RouterLink>
        <span v-else class="button is-rounded" disabled>
            <span class="icon"><font-awesome-icon icon="fa-solid fa-caret-right" /></span>
        </span>
    </div>
</template>
