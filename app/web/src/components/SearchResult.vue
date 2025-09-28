<script setup lang="ts">
import { inject, ref, watch } from "vue";
import {
    getPageItemGroups,
    getSuraNameAR,
    toArabicNumber,
    type PageItem,
} from "../utils/quranize";
import type { SearchResult } from "../utils/types";
import MarkedQuranText from "../components/MarkedQuranText.vue";
import AyaNumber from "../components/AyaNumber.vue";
import { onBeforeRouteLeave } from "vue-router";

const { result } = defineProps<{ result: SearchResult }>();

const isTranslationVisible = ref(false);
const isQuranPageVisible = ref(false);
const pageItemGroups = ref<PageItem[][]>([]);
const textID = ref("");

inject<Function>("getTextID")?.(result.sura, result.aya).then(
    (v: string) => (textID.value = v),
);

function toggleTranslationVisibility() {
    isTranslationVisible.value = !isTranslationVisible.value;
}

function openQuranPage() {
    isQuranPageVisible.value = true;
}

function closeQuranPage() {
    isQuranPageVisible.value = false;
}

watch(
    isQuranPageVisible,
    async () => (pageItemGroups.value = await getPageItemGroups(result.page)),
    { once: true },
);

const toQuranPage = {
    name: "QuranPage",
    params: { page: result.page },
    query: { markedSura: result.sura, markedAya: result.aya },
};

onBeforeRouteLeave((to) => {
    if (isQuranPageVisible.value && to.name === "Home") {
        closeQuranPage();
        return false;
    }
});
</script>

<template>
    <div class="card">
        <header class="card-header" dir="rtl">
            <p
                class="card-header-title quran-text is-size-5 is-clickable"
                @click="openQuranPage"
            >
                <span class="icon-text">
                    <span class="icon">
                        <font-awesome-icon icon="fa-solid fa-book-open" />
                    </span>
                    {{ getSuraNameAR(result.sura) }} :
                    {{ toArabicNumber(result.aya) }}
                </span>
            </p>
            <button
                class="card-header-icon"
                @click="toggleTranslationVisibility"
            >
                <span class="icon">
                    <font-awesome-icon
                        :icon="[
                            'fas',
                            isTranslationVisible ? 'angle-up' : 'angle-down',
                        ]"
                    />
                </span>
            </button>
        </header>
        <div class="card-content">
            <div class="content">
                <p
                    dir="rtl"
                    class="quran-text is-size-5 is-clickable"
                    @click="toggleTranslationVisibility"
                >
                    <MarkedQuranText
                        :beforeMarked="result.before_text"
                        :marked="result.text"
                        :afterMarked="result.after_text"
                    />
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
                    <RouterLink :to="toQuranPage" class="tag is-medium">
                        <span class="icon">
                            <font-awesome-icon
                                icon="fa-solid fa-up-right-and-down-left-from-center"
                            />
                        </span>
                    </RouterLink>
                </p>
                <button
                    class="delete"
                    aria-label="close"
                    @click="closeQuranPage"
                ></button>
            </header>
            <section class="modal-card-body">
                <div
                    dir="rtl"
                    class="quran-text is-size-5"
                    v-for="items in pageItemGroups"
                >
                    <p
                        class="has-text-centered has-text-weight-semibold"
                        v-if="items[0]?.aya === 1"
                    >
                        سورة {{ getSuraNameAR(items[0].sura) }}
                    </p>
                    <p class="has-text-justified">
                        <span v-for="item in items">
                            <MarkedQuranText
                                v-if="
                                    item.sura === result.sura &&
                                    item.aya === result.aya
                                "
                                :beforeMarked="result.before_text"
                                :marked="result.text"
                                :afterMarked="result.after_text"
                            />
                            <span v-else>{{ item.text }}</span>
                            <AyaNumber :aya="item.aya" />
                        </span>
                    </p>
                </div>
            </section>
        </div>
    </div>
</template>
