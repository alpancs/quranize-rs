<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';

type Theme = 0 | 1 | 2; // 0: system, 1: light, 2: dark

const themes = [
  { name: 'system', icon: 'desktop', colorClass: '' },
  { name: 'light', icon: 'sun', colorClass: 'has-text-warning' },
  { name: 'dark', icon: 'moon', colorClass: 'has-text-link-light' },
];

const theme = ref<Theme>(0);
const themeIcon = computed(() => themes[theme.value]?.icon);
const themeColorClass = computed(() => themes[theme.value]?.colorClass);

function switchTheme() {
  setTheme(((theme.value + 1) % themes.length) as Theme);
}

function setTheme(newTheme: Theme) {
  theme.value = newTheme;
  if (newTheme === 0) {
    document.documentElement.removeAttribute('data-theme');
  } else {
    document.documentElement.setAttribute('data-theme', themes[newTheme]?.name);
  }
  localStorage.setItem('theme', newTheme.toString());
}

onMounted(() => {
  setTheme((parseInt(localStorage.getItem('theme')!) || 0) as Theme);
});
</script>

<template>
  <header class="hero is-small is-info">
    <div class="hero-body">
      <div class="container is-max-desktop">
        <div class="level is-mobile">
          <span class="button is-rounded is-invisible">
            <span class="icon"></span>
          </span>
          <div class="level-item">
            <RouterLink to="/" class="title">Quranize</RouterLink>
          </div>
          <div class="level-right">
            <div class="level-item">
              <button class="button is-rounded" @click="switchTheme">
                <span class="icon" :class="themeColorClass">
                  <font-awesome-icon :icon="['fas', themeIcon]" />
                </span>
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </header>
</template>
