<script setup lang="ts">
defineProps<{ beforeMarked: string; marked: string; afterMarked: string }>();
</script>

<template>
    <span>{{ beforeMarked }}</span>
    <mark>{{ marked }}</mark>
    <span>{{ afterMarked }}</span>
</template>
