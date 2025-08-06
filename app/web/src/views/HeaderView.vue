<script setup lang="ts">
import { ref, computed, inject, type Ref } from 'vue';
import { useRoute } from 'vue-router';

const themes = {
  system: { icon: 'desktop', colorClass: '' },
  light: { icon: 'sun', colorClass: 'has-text-warning' },
  dark: { icon: 'moon', colorClass: 'has-text-link-light' },
};
type Theme = keyof typeof themes;

const theme = ref<Theme>('system');
const themeIcon = computed(() => themes[theme.value]?.icon);
const themeColorClass = computed(() => themes[theme.value]?.colorClass);

function switchTheme() {
  if (theme.value === 'system') setTheme('light');
  else if (theme.value === 'light') setTheme('dark');
  else if (theme.value === 'dark') setTheme('system');
}

function setTheme(newTheme: Theme) {
  if (newTheme === 'light' || newTheme === 'dark') {
    document.documentElement.setAttribute('data-theme', newTheme);
  } else {
    newTheme = 'system';
    document.documentElement.removeAttribute('data-theme');
  }
  theme.value = newTheme;
  localStorage.setItem('theme', newTheme);
}

setTheme(localStorage.getItem('theme') as Theme);

const route = useRoute();
const inQuranPage = computed(() => route.path === '/quran-page');
const lang = inject<Ref<string>>('lang');
</script>

<template>
  <header class="hero is-small is-info">
    <div class="hero-body">
      <div class="container is-max-desktop">
        <div class="level is-mobile">

          <div class="level-left">
            <div class="level-item">
              <button class="button is-rounded" @click="switchTheme">
                <span class="icon" :class="themeColorClass">
                  <font-awesome-icon :icon="['fas', themeIcon]" />
                </span>
              </button>
            </div>
          </div>

          <div class="level-item">
            <RouterLink to="/" class="title">Quranize</RouterLink>
          </div>

          <div class="level-right">
            <div class="level-item">
              <div class="tags has-addons" :class="{ 'is-invisible': !inQuranPage }">
                <button v-for="l in ['ar', 'id']" class="tag is-rounded is-uppercase has-text-weight-semibold"
                  :class="{ 'is-primary': lang === l }" @click="lang = l">{{ l }}</button>
              </div>
            </div>
          </div>

        </div>
      </div>
    </div>
  </header>
</template>
