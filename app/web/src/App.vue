<script setup lang="ts">
import HeaderView from './views/HeaderView.vue';
</script>

<template>
  <HeaderView />
  <section class="section pt-4">
    <div class="container is-max-desktop">
      <RouterView v-slot="{ Component }">
        <KeepAlive include="HomeView">
          <component :is="Component" />
        </KeepAlive>
      </RouterView>
    </div>
  </section>
</template>
