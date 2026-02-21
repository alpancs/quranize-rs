<script setup lang="ts">
import { toArabicNumber } from "../utils/quranize";

defineProps<{ aya: number }>();
</script>

<template>
    <span>&#xFD3F;{{ toArabicNumber(aya) }}&#xFD3E;</span>
</template>
