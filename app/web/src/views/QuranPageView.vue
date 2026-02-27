<script setup lang="ts">
import { inject, ref, watch, useTemplateRef, type Ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import { useSwipe } from "@vueuse/core";
import { getSuraNameAR, getSuraNameID, getPageItemGroups, type PageItem } from "../utils/quranize";
import AyaNumber from "../components/AyaNumber.vue";
import QuranPageNav from "../components/QuranPageNav.vue";

type PageItemExt = PageItem & { textID?: string };

const route = useRoute();
const markedSura = parseInt(route.query.markedSura as string);
const markedAya = parseInt(route.query.markedAya as string);

const page = ref(0);
const pageItemGroups = ref<PageItemExt[][]>([]);

const lang = inject<Ref<string>>("lang", ref("ar"));
const getTextID = inject<Function>("getTextID");

watch(
    () => route.params.page,
    async (newValue) => {
        page.value = parseInt(newValue as string);
        pageItemGroups.value = await getPageItemGroups(page.value);
    },
    { immediate: true },
);

watch(pageItemGroups, (newValue) =>
    newValue.forEach((items) =>
        items.forEach(
            async (item) =>
                (item.textID = await getTextID?.(item.sura, item.aya)),
        ),
    ),
);

const router = useRouter();
const { direction } = useSwipe(useTemplateRef("quran-page"));
watch(direction, (d) => {
    if (d === "right" && page.value < 604)
        router.push({ params: { page: page.value + 1 }, query: route.query });
    if (d === "left" && page.value > 1)
        router.push({ params: { page: page.value - 1 }, query: route.query });
});

const needMark = (item: PageItem) =>
    item.sura === markedSura && item.aya === markedAya;
</script>

<template>
    <div class="block" ref="quran-page">
        <div :dir="lang === 'ar' ? 'rtl' : 'ltr'">
            <div v-for="items in pageItemGroups">
                <p class="has-text-centered is-size-5-touch is-size-4-desktop has-text-weight-bold"
                    v-if="items[0]!.aya === 1">
                    <span class="tag is-large is-rounded">
                        <span v-if="lang === 'ar'" class="quran-text">سورة {{ getSuraNameAR(items[0]!.sura) }}</span>
                        <span v-else>Surah {{ getSuraNameID(items[0]!.sura) }}</span>
                    </span>
                </p>
                <p class="has-text-justified is-size-5-touch is-size-4-desktop">
                    <span v-for="item in items">
                        <span v-if="lang === 'ar'" class="quran-text quran-paragraph">
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

    <QuranPageNav v-if="pageItemGroups.length > 0" :page :lang></QuranPageNav>
</template>
