<script setup lang="ts">
import { useRoute } from 'vue-router'

const route = useRoute()
const { q } = route.query
</script>

<template>
    <div class="box">
        <p class="quran-text subtitle is-5 is-spaced">{{ q }}</p>
    </div>
</template>

<style scoped></style>
