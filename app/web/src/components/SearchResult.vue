<script setup lang="ts">
import type { EncodeResult } from "../utils/types";

const props = defineProps<{ result: EncodeResult }>();
const link = { name: "SearchResult", query: props.result };
</script>

<template>
    <RouterLink :to="link" class="box" dir="rtl">
        <div class="is-flex is-justify-content-space-between is-align-items-center">
            <p class="quran-text has-text-weight-semibold">
                {{ result.quran }}
            </p>
            <span class="tag is-rounded">{{ result.location_count }}</span>
        </div>
    </RouterLink>
</template>
