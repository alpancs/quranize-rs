<script setup lang="ts">
import { provide, ref } from 'vue';
import HeaderView from './views/HeaderView.vue';

provide('lang', ref('ar'));
</script>

<template>
  <HeaderView />
  <section class="section pt-5">
    <div class="container is-max-desktop">
      <RouterView v-slot="{ Component }">
        <KeepAlive include="HomeView">
          <component :is="Component" />
        </KeepAlive>
      </RouterView>
    </div>
  </section>
</template>
