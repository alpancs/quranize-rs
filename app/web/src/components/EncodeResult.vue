<script setup lang="ts">
import ExplanationGroup from "../components/ExplanationGroup.vue";
import type { EncodeResult } from "../utils/types";

defineProps<{ result: EncodeResult }>();
</script>

<template>
    <RouterLink :to="{ name: 'SearchResult', query: { quran: result.quran } }" class="box">
        <div class="is-flex is-flex-direction-column">
            <div class="block is-flex is-justify-content-space-between is-align-items-center">
                <span class="tag is-rounded">{{ result.location_count }}</span>
                <p class="quran-text has-text-weight-semibold is-size-5-touch is-size-4-desktop">
                    {{ result.quran }}
                </p>
            </div>

            <ExplanationGroup :explanations="result.explanations" />
        </div>
    </RouterLink>
</template>
