<script setup lang="ts">
import { computed } from "vue";
import { initiated } from "../utils/quranize";

const keyword = defineModel();
const isLoading = computed(() => !initiated.value);
const showDeleteButton = computed(() => !isLoading.value && keyword.value);
const placeholder = "yaa ayyuhannasuttaquu robbakum";
</script>

<template>
    <div class="control has-icons-left has-icons-right" :class="{ 'is-loading': isLoading }">
        <input class="input is-rounded" type="search" v-model.trim="keyword" :placeholder="placeholder"
            autocorrect="off" autocomplete="off" spellcheck="false" />
        <span class="icon is-left">
            <font-awesome-icon icon="fa-solid fa-search" />
        </span>
        <span class="icon is-right" v-show="showDeleteButton" @click="keyword = ''">
            <button class="delete is-small"></button>
        </span>
    </div>
</template>
