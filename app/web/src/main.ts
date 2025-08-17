import { createApp } from 'vue';
import { router } from './router';
import './style.css';
import App from './App.vue';
import { library } from '@fortawesome/fontawesome-svg-core';
import * as fas from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';

library.add(
    fas.faSearch,
    fas.faDesktop, fas.faMoon, fas.faSun,
    fas.faAngleDown, fas.faAngleUp, fas.faBookOpen, fas.faUpRightAndDownLeftFromCenter,
    fas.faCaretLeft, fas.faCaretRight,
);

createApp(App).use(router).component('font-awesome-icon', FontAwesomeIcon).mount('#app');

if ('serviceWorker' in navigator) {
    window.addEventListener('load', () => {
        navigator.serviceWorker.register('/service-worker.js');
    });
}
