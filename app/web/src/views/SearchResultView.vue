<script setup lang="ts">
import { ref, inject, type Ref } from 'vue';
import { useRoute, type LocationQueryValue } from 'vue-router';
import SearchResult from '../components/SearchResult.vue';
import type { SearchResult as SR, Explanation } from '../utils/types';

const initiated = inject<Ref<boolean>>('quranize.initiated');
const search = inject<(quran: string) => Promise<SR[]>>('quranize.search');
const explain = inject<(quran: string, expl: string) => Promise<Explanation[]>>('quranize.explain');

const searchResults = ref<SR[]>([]);
const compactExpls = ref<Explanation[]>([]);

const getString = (v: LocationQueryValue | LocationQueryValue[]) => (Array.isArray(v) ? v[0] : v) ?? '';
const route = useRoute();
const quran = getString(route.query.quran);
const expl = getString(route.query.explanation);

search?.(quran).then((v) => searchResults.value = v);
explain?.(quran, expl).then((v) => compactExpls.value = v);
</script>

<template>
    <div class="block">
        <p class="quran-text title is-4 has-text-centered">{{ quran }}</p>
        <div class="skeleton-block" v-if="!initiated"></div>
        <div class="field is-grouped is-grouped-multiline is-justify-content-center">
            <div class="control" v-for="e in compactExpls">
                <div class="tags has-addons">
                    <span class="tag is-info">{{ e.alphabet }}</span>
                    <span class="tag"><span class="quran-text">{{ e.quran }}</span></span>
                </div>
            </div>
        </div>
    </div>
    <div class="skeleton-block" v-if="!initiated"></div>
    <SearchResult :result v-for="result in searchResults" />
</template>
