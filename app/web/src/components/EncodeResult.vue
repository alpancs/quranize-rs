<script setup lang="ts">
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
            <div class="field is-grouped is-grouped-multiline is-justify-content-center">
                <div class="control" v-for="expl in result.explanations">
                    <div class="tags has-addons">
                        <span class="tag is-info">{{ expl.alphabet }}</span>
                        <span class="tag">
                            <span class="quran-text">{{ expl.quran }}</span>
                        </span>
                    </div>
                </div>
            </div>
        </div>
    </RouterLink>
</template>
