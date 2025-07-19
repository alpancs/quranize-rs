<script setup lang="ts">
import { inject, type Ref } from 'vue';

const keyword = defineModel();
const placeholder = 'masyaallah';
const initialized = inject<Ref<boolean>>('quranize.initialized');
</script>

<template>
    <div class="control has-icons-left" :class="{ 'is-loading': !initialized }">
        <input class="input is-rounded" type="search" v-model.trim="keyword" :placeholder="placeholder"
            spellcheck="false" autofocus />
        <span class="icon is-left">🔍</span>
    </div>
</template>
