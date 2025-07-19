<script setup lang="ts">
import { ref, inject } from 'vue';
import { useRoute } from 'vue-router';
import SearchResult from '../components/SearchResult.vue';
import type { SearchResult as SR, Explanation } from '../utils/types';

const search = inject<(quran: string) => Promise<SR[]>>('quranize.search');
const explain = inject<(quran: string, expl: string) => Promise<Explanation[]>>('quranize.explain');

const searchResults = ref<SR[]>([]);
const compactExpls = ref<Explanation[]>([]);

const route = useRoute();
const { q, e } = route.query;
const quran = (Array.isArray(q) ? q[0] : q) ?? '';
const expl = (Array.isArray(e) ? e[0] : e) ?? '';

search?.(quran).then((v) => searchResults.value = v);
explain?.(quran, expl).then((v) => compactExpls.value = v);
</script>

<template>
    <div class="block">
        <p class="quran-text title is-4 has-text-centered">{{ q }}</p>
        <div class="field is-grouped is-grouped-multiline is-justify-content-center">
            <div class="control" v-for="e in compactExpls">
                <div class="tags has-addons">
                    <span class="tag is-info">{{ e.alphabet }}</span>
                    <span class="tag"><span class="quran-text">{{ e.quran }}</span></span>
                </div>
            </div>
        </div>
    </div>

    <SearchResult :result v-for="result in searchResults" />
</template>
