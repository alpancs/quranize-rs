<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { call } from "../utils/quranize";
import type { EncodeResult, Explanation } from "../utils/types";

const props = defineProps<{ result: EncodeResult }>();
const searchResultLink = computed(() => ({ name: "SearchResult", query: { quran: props.result.quran } }));
const explanations = ref<Explanation[]>([]);

watch(
    () => props.result,
    async (result) => {
        explanations.value = await call<Explanation[]>("compressExpl", result.quran, result.explanation);
    },
    { immediate: true },
);
</script>

<template>
    <RouterLink :to="searchResultLink" class="box">
        <div class="is-flex is-flex-direction-column">
            <div class="is-flex is-justify-content-space-between is-align-items-center">
                <span class="tag is-rounded">{{ result.location_count }}</span>
                <p class="quran-text has-text-weight-semibold is-size-5-touch is-size-4-desktop">
                    {{ result.quran }}
                </p>
            </div>
            <div class="field is-grouped is-grouped-multiline is-justify-content-center">
                <div class="control" v-for="expl in explanations">
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
