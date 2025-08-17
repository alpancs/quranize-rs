<script setup lang="ts">
import { computed, inject, ref, watch, type Ref } from 'vue';
import { useRoute } from 'vue-router';
import { toArabicNumber, getSuraNameAR, getSuraNameID, getPageItemGroups, type PageItem } from '../utils/quranize';
import MarkedQuranText from '../components/MarkedQuranText.vue';
import AyaNumber from '../components/AyaNumber.vue';

const route = useRoute();
const page = ref(parseInt(route.query.page as string));
const sura = parseInt(route.query.sura as string);
const aya = parseInt(route.query.aya as string);
const beforeMarked = route.query.before_text as string;
const marked = route.query.text as string;
const afterMarked = route.query.after_text as string;

const pageItemGroups = ref<PageItem[][]>([]);
getPageItemGroups(page.value).then((v) => pageItemGroups.value = v);

watch(() => route.query.page, async (newPage) => {
    page.value = parseInt(newPage as string);
    pageItemGroups.value = await getPageItemGroups(page.value);
});

const lang = inject<Ref<string>>('lang');
const isAR = computed(() => lang?.value === 'ar');
const isID = computed(() => lang?.value === 'id');
const getTextID = inject<Function>('getTextID');

const needMark = (item: PageItem) => item.sura === sura && item.aya === aya;
</script>

<template>
    <div class="box">
        <div v-if="isAR" dir="rtl">
            <div v-for="items in pageItemGroups">
                <p class="has-text-centered mt-4 has-text-weight-semibold quran-text" v-if="items[0].aya === 1">
                    سورة {{ getSuraNameAR(items[0].sura) }}
                </p>
                <p class="has-text-justified">
                    <span v-for="item in items">
                        <MarkedQuranText v-if="needMark(item)" :beforeMarked :marked :afterMarked />
                        <span v-else class="quran-text">{{ item.text }}</span>
                        <AyaNumber :aya="item.aya" />
                    </span>
                </p>
            </div>
        </div>
        <div v-if="isID" class="has-text-justified">
            <div v-for="items in pageItemGroups">
                <p class="has-text-centered mt-4 has-text-weight-semibold" v-if="items[0].aya === 1">
                    Surah {{ getSuraNameID(items[0].sura) }}
                </p>
                <p class="has-text-justified">
                    <span v-for="item in items">
                        ({{ item.aya }}) {{ getTextID?.(item.sura, item.aya) }}
                    </span>
                </p>
            </div>
        </div>
    </div>

    <nav class="tags has-addons is-centered">
        <RouterLink class="tag is-rounded" v-if="page < 604"
            :to="{ query: { page: page + 1, sura, aya, before_text: beforeMarked, text: marked, after_text: afterMarked } }">
            <span class="icon"><font-awesome-icon icon="fa-solid fa-caret-left" /></span>
            <span v-if="isAR" class="quran-text">{{ toArabicNumber(page + 1) }}</span>
            <span v-else>{{ page + 1 }}</span>
        </RouterLink>
        <span v-else class="tag is-rounded" disabled>
            <span class="icon"><font-awesome-icon icon="fa-solid fa-caret-left" /></span>
        </span>

        <button class="tag is-primary has-text-weight-bold">
            <span v-if="isAR" class="quran-text">{{ toArabicNumber(page) }}</span>
            <span v-else>{{ page }}</span>
        </button>

        <RouterLink class="tag is-rounded" v-if="page > 1"
            :to="{ query: { page: page - 1, sura, aya, before_text: beforeMarked, text: marked, after_text: afterMarked } }">
            <span v-if="isAR" class="quran-text">{{ toArabicNumber(page - 1) }}</span>
            <span v-else>{{ page - 1 }}</span>
            <span class="icon"><font-awesome-icon icon="fa-solid fa-caret-right" /></span>
        </RouterLink>
        <span v-else class="tag is-rounded" disabled>
            <span class="icon"><font-awesome-icon icon="fa-solid fa-caret-right" /></span>
        </span>
    </nav>
</template>
