<script setup lang="ts">
import HeaderView from './views/HeaderView.vue'
</script>

<template>
  <HeaderView />
  <section class="section">
    <div class="container is-max-widescreen">
      <RouterView />
    </div>
  </section>
</template>

<style scoped></style>
