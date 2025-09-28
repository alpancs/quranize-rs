<script setup lang="ts">
import { computed, inject, ref, watch, useTemplateRef, type Ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import { useSwipe } from "@vueuse/core";
import {
    toArabicNumber,
    getSuraNameAR,
    getSuraNameID,
    getPageItemGroups,
    type PageItem,
} from "../utils/quranize";
import AyaNumber from "../components/AyaNumber.vue";

type PageItemExt = PageItem & { textID?: string };

const route = useRoute();
const markedSura = parseInt(route.query.markedSura as string);
const markedAya = parseInt(route.query.markedAya as string);

const page = ref(0);
const pageItemGroups = ref<PageItemExt[][]>([]);

const lang = inject<Ref<string>>("lang");
const isAR = computed(() => lang?.value === "ar");
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
        <div :dir="isAR ? 'rtl' : 'ltr'">
            <div v-for="items in pageItemGroups">
                <p
                    v-if="items[0]!.aya === 1"
                    class="has-text-centered has-text-weight-semibold"
                >
                    <span v-if="isAR" class="quran-text is-size-5"
                        >سورة {{ getSuraNameAR(items[0]!.sura) }}</span
                    >
                    <span v-else
                        >Surah {{ getSuraNameID(items[0]!.sura) }}</span
                    >
                </p>
                <p class="has-text-justified">
                    <span v-for="item in items">
                        <span v-if="isAR" class="quran-text is-size-5">
                            <component :is="needMark(item) ? 'mark' : 'span'">{{
                                item.text
                            }}</component>
                            <AyaNumber :aya="item.aya" />
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

    <nav class="tags has-addons is-centered">
        <RouterLink
            :to="{ params: { page: page + 1 }, query: route.query }"
            v-if="page < 604"
            class="tag is-rounded"
        >
            <span class="icon"
                ><font-awesome-icon icon="fa-solid fa-caret-left"
            /></span>
            <span v-if="isAR" class="quran-text">{{
                toArabicNumber(page + 1)
            }}</span>
            <span v-else>{{ page + 1 }}</span>
        </RouterLink>
        <span v-else class="tag is-rounded" disabled>
            <span class="icon"
                ><font-awesome-icon icon="fa-solid fa-caret-left"
            /></span>
        </span>

        <button class="tag is-primary has-text-weight-bold">
            <span v-if="isAR" class="quran-text">{{
                toArabicNumber(page)
            }}</span>
            <span v-else>{{ page }}</span>
        </button>

        <RouterLink
            :to="{ params: { page: page - 1 }, query: route.query }"
            v-if="page > 1"
            class="tag is-rounded"
        >
            <span v-if="isAR" class="quran-text">{{
                toArabicNumber(page - 1)
            }}</span>
            <span v-else>{{ page - 1 }}</span>
            <span class="icon"
                ><font-awesome-icon icon="fa-solid fa-caret-right"
            /></span>
        </RouterLink>
        <span v-else class="tag is-rounded" disabled>
            <span class="icon"
                ><font-awesome-icon icon="fa-solid fa-caret-right"
            /></span>
        </span>
    </nav>
</template>
