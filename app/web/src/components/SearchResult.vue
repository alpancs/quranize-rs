<script setup lang="ts">
import { computed } from "vue";
import type { EncodeResult } from "../utils/types";

const { result } = defineProps<{ result: EncodeResult }>();
const searchResultLink = computed(() => ({
    name: "SearchResult",
    query: { quran: result.quran, explanation: result.explanation },
}));
</script>

<template>
    <RouterLink :to="searchResultLink" class="box" dir="rtl">
        <div class="is-flex is-justify-content-space-between is-align-items-center">
            <p class="quran-text has-text-weight-semibold">
                {{ result.quran }}
            </p>
            <span class="tag is-rounded">{{ result.location_count }}</span>
        </div>
    </RouterLink>
</template>
