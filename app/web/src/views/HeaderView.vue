<script setup lang="ts">
import { ref, onMounted } from 'vue';

type Theme = 'light' | 'dark' | 'auto';

const isBurgerActive = ref(false);
const isThemeMenuActive = ref(false);
const theme = ref<Theme>('auto');

function setTheme(newTheme: Theme) {
  theme.value = newTheme;
  if (newTheme === 'auto') {
    document.documentElement.removeAttribute('data-theme');
  } else {
    document.documentElement.setAttribute('data-theme', newTheme);
  }
  localStorage.setItem('theme', newTheme);
}

onMounted(() => {
  const savedTheme = (localStorage.getItem('theme') as Theme) || 'auto';
  setTheme(savedTheme);
});
</script>

<template>
  <nav class="navbar is-info is-fixed-top has-shadow">
    <div class="container is-max-desktop">
      <div class="navbar-brand">
        <div class="navbar-end">
          <RouterLink to="/" class="navbar-item">
            <span class="has-text-weight-extrabold is-size-3">Quranize</span>
          </RouterLink>
        </div>
        <a role="button" class="navbar-burger" :class="{ 'is-active': isBurgerActive }" aria-label="menu"
          aria-expanded="false" data-target="navbarBasicExample" @click="isBurgerActive = !isBurgerActive">
          <span aria-hidden="true"></span>
          <span aria-hidden="true"></span>
          <span aria-hidden="true"></span>
          <span aria-hidden="true"></span>
        </a>
      </div>
      <div class="navbar-menu" :class="{ 'is-active': isBurgerActive }">
        <div class="navbar-end">
          <div class="navbar-item has-dropdown" :class="{ 'is-active': isThemeMenuActive }"
            @mouseenter="isThemeMenuActive = true" @mouseleave="isThemeMenuActive = false">
            <a class="navbar-link">
              Theme
            </a>
            <div class="navbar-dropdown is-right">
              <a class="navbar-item" :class="{ 'is-active': theme === 'light' }" @click="setTheme('light')">☀️ Light</a>
              <a class="navbar-item" :class="{ 'is-active': theme === 'auto' }" @click="setTheme('auto')">✨ Auto</a>
              <a class="navbar-item" :class="{ 'is-active': theme === 'dark' }" @click="setTheme('dark')">🌙 Dark</a>
            </div>
          </div>
        </div>
      </div>
    </div>
  </nav>
</template>
