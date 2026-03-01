<script setup lang="ts">
import ExplanationGroup from "../components/ExplanationGroup.vue";
import type { EncodeResult } from "../utils/types";

function encode(str: string): string {
    const bytes = new TextEncoder().encode(str);
    const binString = String.fromCodePoint(...bytes);
    return btoa(binString);
}

defineProps<{ result: EncodeResult }>();
</script>

<template>
    <RouterLink class="box" :to="{ name: 'SearchResult', query: { code: encode(result.quran) } }">
        <div class="block is-flex is-justify-content-space-between is-align-items-center">
            <span class="tag is-rounded">{{ result.location_count }}</span>
            <p class="quran-text has-text-weight-semibold is-size-5-touch is-size-4-desktop">
                {{ result.quran }}
            </p>
        </div>
        <ExplanationGroup :explanations="result.explanations" />
    </RouterLink>
</template>
