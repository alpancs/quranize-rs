import { createApp } from 'vue'
import { createRouter, createWebHistory } from 'vue-router'
import './style.css'
import App from './App.vue'
import HomeView from './views/HomeView.vue'

const router = createRouter({
    history: createWebHistory(),
    routes: [
        { path: '/', component: HomeView },
    ],
})

const app = createApp(App)
app.use(router)
app.mount('#app')
app.provide('quranizeWorker', new Worker("/src/workers/quranize", { type: "module" }))
