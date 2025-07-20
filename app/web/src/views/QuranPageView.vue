<script setup lang="ts">
import { inject, ref } from 'vue';
import { useRoute, type LocationQueryValue } from 'vue-router';

const getQuran = inject<(index: number) => Promise<string>>('quranize.getQuran');

const getString = (v: LocationQueryValue | LocationQueryValue[]) => (Array.isArray(v) ? v[0] : v) ?? '';
const route = useRoute();
const quran = ref('');
const quran1 = ref('');
const index = parseInt(getString(route.query.index));
getQuran?.(index).then((v) => quran.value = v);
getQuran?.(index + 1).then((v) => quran1.value = v);
</script>

<template>
    <div class="title is-1 has-text-centered">🚧</div>
    <p class="block quran-text">{{ quran }}</p>
    <p class="block quran-text">{{ quran1 }}</p>
</template>
