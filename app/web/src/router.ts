import { createRouter, createWebHistory } from 'vue-router';
import HomeView from './views/HomeView.vue';
import SearchResultView from './views/SearchResultView.vue';
import QuranPageView from './views/QuranPageView.vue';

export const router = createRouter({
    history: createWebHistory(),
    routes: [
        { path: '/', component: HomeView },
        { path: '/search-result', component: SearchResultView },
        { path: '/quran-page', component: QuranPageView },
    ],
});
