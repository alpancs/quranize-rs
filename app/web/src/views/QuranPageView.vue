<script setup lang="ts">
import { useSwipe } from "@vueuse/core";
import { computed, inject, ref, useTemplateRef, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import AyaNumber from "../components/AyaNumber.vue";
import QuranPageNav from "../components/QuranPageNav.vue";
import { getPageItemGroups, getSuraNameAR, getSuraNameID, initiated, lang, type PageItem } from "../utils/quranize";

type PageItemExt = PageItem & { textID?: string };

const route = useRoute();
const markedSura = computed(() => parseInt(route.query.markedSura as string));
const markedAya = computed(() => parseInt(route.query.markedAya as string));

const page = ref(0);
const pageItemGroups = ref<PageItemExt[][]>([]);

const getTextID = inject<Function>("getTextID");

watch(
    () => route.params.page,
    async (newValue) => {
        page.value = parseInt(newValue as string);
        pageItemGroups.value = await getPageItemGroups(page.value);
        pageItemGroups.value.flat().forEach(async (item) => item.textID = await getTextID?.(item.sura, item.aya));
    },
    { immediate: true },
);

const router = useRouter();
const { direction } = useSwipe(useTemplateRef("quran-page"));
watch(direction, (d) => {
    if (d === "right" && page.value < 604) router.push({ params: { page: page.value + 1 }, query: route.query });
    else if (d === "left" && page.value > 1) router.push({ params: { page: page.value - 1 }, query: route.query });
});

const needMark = (item: PageItem) => item.sura === markedSura.value && item.aya === markedAya.value;
</script>

<template>
    <div class="skeleton-block" v-if="!initiated"></div>

    <div class="block" ref="quran-page">
        <div :dir="lang === 'ar' ? 'rtl' : 'ltr'">
            <div v-for="items in pageItemGroups">
                <p class="has-text-centered has-text-weight-semibold mt-4 mb-2" v-if="items[0]!.aya === 1">
                    <span class="tag is-medium is-rounded">
                        <span v-if="lang === 'ar'" class="quran-text is-size-5-touch is-size-4-desktop">
                            سورة {{ getSuraNameAR(items[0]!.sura) }}
                        </span>
                        <span v-else>Surah {{ getSuraNameID(items[0]!.sura) }}</span>
                    </span>
                </p>
                <p class="has-text-justified">
                    <span v-for="item in items">
                        <span v-if="lang === 'ar'" class="quran-text quran-paragraph is-size-5-touch is-size-4-desktop">
                            <component :is="needMark(item) ? 'mark' : 'span'">
                                {{ item.text }}
                            </component>
                            <AyaNumber class="ml-2 mr-1" :aya="item.aya" />
                        </span>
                        <span v-else class="translation">
                            ({{ item.aya }})
                            <component :is="needMark(item) ? 'mark' : 'span'">
                                {{ item.textID }}
                            </component>
                        </span>
                    </span>
                </p>
            </div>
        </div>
    </div>

    <QuranPageNav :page :lang />
</template>
