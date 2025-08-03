<script setup lang="ts">
import { initiated } from '../utils/quranize';

const keyword = defineModel();
const placeholder = 'masyaallah';
</script>

<template>
    <div class="control has-icons-left" :class="{ 'is-loading': !initiated }">
        <input class="input is-rounded" type="search" v-model.trim="keyword" :placeholder="placeholder"
            spellcheck="false" autofocus />
        <span class="icon is-left">
            <font-awesome-icon :icon="['fas', 'search']" />
        </span>
    </div>
</template>
