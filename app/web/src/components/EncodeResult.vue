<script setup lang="ts">
import type { EncodeResult } from '../utils/types';

defineProps<{ result: EncodeResult }>();
</script>

<template>
    <RouterLink :to="{ path: '/search-result', query: result }" class="box" dir="rtl">
        <div class="is-flex is-justify-content-space-between is-align-items-center">
            <p class="quran-text is-size-5 has-text-weight-semibold">{{ result.quran }}</p>
            <span class="tag is-rounded">{{ result.location_count }}</span>
        </div>
    </RouterLink>
</template>
