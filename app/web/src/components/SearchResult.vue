<script setup lang="ts">
defineProps(['searchResults'])
</script>

<template>
    <RouterLink :to="{ path: '/result', query: { q: result.quran, e: result.explanation } }"
        v-for="result in searchResults" class="box" dir="rtl">
        <div class="is-flex is-justify-content-space-between is-align-items-center">
            <p class="quran-text is-size-5 has-text-weight-semibold">{{ result.quran }}</p>
            <span class="tag is-rounded">{{ result.location_count }}</span>
        </div>
    </RouterLink>
</template>
