import { createApp } from 'vue';
import { router } from './router';
import './style.css';
import App from './App.vue';
import { library } from '@fortawesome/fontawesome-svg-core';
import { faCaretLeft, faCaretRight, faDesktop, faMoon, faSearch, faSun } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';

library.add(faSearch, faDesktop, faSun, faMoon, faCaretRight, faCaretLeft);

createApp(App).use(router).component('font-awesome-icon', FontAwesomeIcon).mount('#app');

if ('serviceWorker' in navigator) {
    window.addEventListener('load', () => {
        navigator.serviceWorker.register('/service-worker.js');
    });
}
