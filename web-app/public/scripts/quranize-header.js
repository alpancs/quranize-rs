import { createApp } from "./vue.esm-browser.js";

createApp({
    data() {
        return {
            darkMode: false,
        };
    },
    methods: {
        toggleTheme() {
            this.darkMode ^= true;
            if (this.darkMode) document.documentElement.setAttribute("data-theme", "dark");
            else document.documentElement.setAttribute("data-theme", "light");
        },
    },
    mounted() {
        window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)')
            .addEventListener("change", event => this.darkMode = event.matches);
    },
}).mount("#quranize-header");
