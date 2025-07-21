<script setup lang="ts">
import { ref } from 'vue';
import { useRoute } from 'vue-router';
import { initiated, call } from '../utils/quranize';
import type { SearchResult as SR, Explanation } from '../utils/types';
import SearchResult from '../components/SearchResult.vue';

const route = useRoute();
const { quran, explanation: expl } = route.query;

const searchResults = ref<SR[]>([]);
const compactExpls = ref<Explanation[]>([]);

call<SR[]>('getLocations', quran).then((v) => searchResults.value = v);
call<Explanation[]>('compressExpl', quran, expl).then((v) => compactExpls.value = v);
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
