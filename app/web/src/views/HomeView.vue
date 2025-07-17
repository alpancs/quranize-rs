<script setup lang="ts">
import SearchBar from '../components/SearchBar.vue'
import SearchResult from '../components/SearchResult.vue'

const searchResults = ['no one', 'else']
</script>

<template>
    <div class="block">
        <SearchBar />
    </div>
    <div class="block">
        <SearchResult v-for="result in searchResults" :result />
    </div>
</template>

<style scoped></style>
