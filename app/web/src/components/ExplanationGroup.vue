<script setup lang="ts">
import type { Explanation } from "../utils/types";
defineProps<{ explanations: Explanation[] }>();
</script>

<template>
  <div class="field is-grouped is-grouped-multiline is-justify-content-center">
    <div class="control" v-for="expl in explanations" :key="expl.alphabet + expl.quran">
      <div class="tags has-addons">
        <span class="tag is-info">{{ expl.alphabet }}</span>
        <span class="tag">
          <span class="quran-text">{{ expl.quran }}</span>
        </span>
      </div>
    </div>
  </div>
</template>
