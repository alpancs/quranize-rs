<script setup lang="ts">
import { provide, ref } from 'vue';
import HeaderView from './views/HeaderView.vue';

const textIDMap = (async function () {
  const map = new Map<string, string>();
  const plainTextID = (await import('./utils/quran/id.indonesian.txt?raw')).default;
  plainTextID.split('\n').forEach((line) => {
    const split = line.split('|');
    if (split.length === 3) {
      const [sura, aya, text] = split;
      map.set(`${sura}.${aya}`, text);
    }
  });
  return map;
})();

provide('lang', ref('ar'));
provide('getTextID', async (sura: number, aya: number) => (await textIDMap).get(`${sura}.${aya}`));
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
