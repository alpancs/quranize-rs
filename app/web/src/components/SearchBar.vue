<script setup lang="ts">
import { inject, type Ref } from 'vue';

const initiated = inject<Ref<boolean>>('quranize.initiated');

const keyword = defineModel();
const placeholder = 'masyaallah';
</script>

<template>
    <div class="control has-icons-left" :class="{ 'is-loading': !initiated }">
        <input class="input is-rounded" type="search" v-model.trim="keyword" :placeholder="placeholder"
            spellcheck="false" autofocus />
        <span class="icon is-left">🔍</span>
    </div>
</template>
