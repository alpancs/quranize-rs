<script setup lang="ts">
import { ref } from 'vue';
import { useRoute } from 'vue-router';
import { call } from '../utils/quranize';

const route = useRoute();
const index = parseInt(route.query.index as string);

const quran = ref('');
const quran1 = ref('');

call<string>('getQuran', index).then((v) => quran.value = v);
call<string>('getQuran', index + 1).then((v) => quran1.value = v);
</script>

<template>
    <div class="title is-1 has-text-centered">🚧</div>
    <p class="block quran-text">{{ quran }}</p>
    <p class="block quran-text">{{ quran1 }}</p>
</template>
