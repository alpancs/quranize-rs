<script setup lang="ts">
import { toArabicNumber } from "../utils/quranize";

const { lang } = defineProps<{ page: number; lang: string | undefined; }>();
const isAR = lang === undefined || lang === "ar";
</script>

<template>
  <nav class="tags are-medium has-addons is-centered">
    <RouterLink :to="{ params: { page: page + 1 }, query: $route.query }" v-if="page < 604" class="tag is-rounded">
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

    <RouterLink :to="{ params: { page: page - 1 }, query: $route.query }" v-if="page > 1" class="tag is-rounded">
      <span v-if="isAR" class="quran-text">{{ toArabicNumber(page - 1) }}</span>
      <span v-else>{{ page - 1 }}</span>
      <span class="icon"><font-awesome-icon icon="fa-solid fa-caret-right" /></span>
    </RouterLink>
    <span v-else class="tag is-rounded" disabled>
      <span class="icon"><font-awesome-icon icon="fa-solid fa-caret-right" /></span>
    </span>
  </nav>
</template>
