<script setup lang="ts">
import { ref } from 'vue'

const workerInitiated = ref(false)
const keyword = ref('')
const ph = 'masyaallah'

setTimeout(() => workerInitiated.value = true, 1200)
</script>

<template>
    <div class="control has-icons-left" :class="{ 'is-loading': !workerInitiated }">
        <input class="input is-rounded" type="search" :placeholder="ph" spellcheck="false" v-model="keyword" />
        <span class="icon is-left">🔍</span>
    </div>
</template>
