import { createApp } from 'vue';
import { router } from './router';
import { useQuranize } from './utils/quranize';
import './style.css';
import App from './App.vue';

const app = createApp(App).use(router);

const { initialized, encode, search, explain } = useQuranize();
app.provide('quranize.initialized', initialized);
app.provide('quranize.encode', encode);
app.provide('quranize.search', search);
app.provide('quranize.explain', explain);

app.mount('#app');
