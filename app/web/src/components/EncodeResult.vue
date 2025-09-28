<script setup lang="ts">
import type { EncodeResult } from "../utils/types";

defineProps<{ result: EncodeResult }>();
</script>

<template>
    <RouterLink
        :to="{ name: 'SearchResult', query: result }"
        class="box"
        dir="rtl"
    >
        <div
            class="is-flex is-justify-content-space-between is-align-items-center"
        >
            <p class="quran-text has-text-weight-semibold">
                {{ result.quran }}
            </p>
            <span class="tag is-rounded">{{ result.location_count }}</span>
        </div>
    </RouterLink>
</template>
