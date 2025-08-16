<script setup lang="ts">
import { provide, ref } from 'vue';
import HeaderView from './views/HeaderView.vue';
import plainTextID from './utils/quran/id.indonesian.txt?raw';

const textIDMap = (function () {
  const map = new Map<string, string>();
  plainTextID.split('\n').forEach((line) => {
    const split = line.split('|');
    if (split.length === 3) {
      const [sura, aya, text] = split;
      map.set(`${sura}.${aya}`, text);
    }
  });
  return map;
})();

function getTextID(sura: number, aya: number) {
  return textIDMap.get(`${sura}.${aya}`);
}

provide('lang', ref('ar'));
provide('getTextID', getTextID);
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
