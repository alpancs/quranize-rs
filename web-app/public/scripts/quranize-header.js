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
}).mount("#quranize-header");
