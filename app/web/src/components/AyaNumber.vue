<script setup lang="ts">
import { toArabicNumber } from "../utils/quranize";

defineProps<{ aya: number }>();
</script>

<template>
    <span class="ml-2 mr-1">&#xFD3F;{{ toArabicNumber(aya) }}&#xFD3E;</span>
</template>
