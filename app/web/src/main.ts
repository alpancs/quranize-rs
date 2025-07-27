import { createApp } from 'vue';
import { router } from './router';
import './style.css';
import App from './App.vue';

createApp(App).use(router).mount('#app');

if ('serviceWorker' in navigator) {
    window.addEventListener('load', () => {
        navigator.serviceWorker.register('/service-worker.js');
    });
}
