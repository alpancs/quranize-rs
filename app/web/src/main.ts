import { createApp } from 'vue';
import { router } from './router';
import { useQuranize } from './utils/quranize';
import './style.css';
import App from './App.vue';

const app = createApp(App).use(router);

const { initiated, encode, search, explain, getQuran } = useQuranize();
app.provide('quranize.initiated', initiated);
app.provide('quranize.encode', encode);
app.provide('quranize.search', search);
app.provide('quranize.explain', explain);
app.provide('quranize.getQuran', getQuran);

app.mount('#app');
