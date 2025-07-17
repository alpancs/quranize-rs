<script setup lang="ts">
import { useRoute } from 'vue-router'

const route = useRoute()
const { q } = route.query
</script>

<template>
    <div class="box">
        {{ q }}
    </div>
</template>

<style scoped></style>
