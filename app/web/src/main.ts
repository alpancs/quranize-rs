import { createApp } from 'vue'
import { createRouter, createWebHistory } from 'vue-router'
import { useQuranize } from './utils/quranize'
import './style.css'
import App from './App.vue'
import HomeView from './views/HomeView.vue'
import ResultView from './views/ResultView.vue'

const router = createRouter({
    history: createWebHistory(),
    routes: [
        { path: '/', component: HomeView },
        { path: '/result', component: ResultView },
    ],
})

const app = createApp(App)
app.use(router)

const { initialized, encode, search, explain } = useQuranize()

app.provide('quranize.initialized', initialized)
app.provide('quranize.encode', encode)
app.provide('quranize.search', search)
app.provide('quranize.explain', explain)

app.mount('#app')
