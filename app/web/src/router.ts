import { createRouter, createWebHistory } from "vue-router";
import HomeView from "./views/HomeView.vue";
import SearchResultView from "./views/SearchResultView.vue";
import QuranPageView from "./views/QuranPageView.vue";

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    { name: "Home", component: HomeView, path: "/" },
    { name: "SearchResult", component: SearchResultView, path: "/search" },
    { name: "QuranPage", component: QuranPageView, path: "/quran/pages/:page" },
  ],
});
