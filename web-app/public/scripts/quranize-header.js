import { createApp } from "./vue.esm-browser.js";

const DarkModeQuery = matchMedia("(prefers-color-scheme: dark)");

createApp({
    data: () => ({ darkMode: undefined }),
    methods: {
        toggleTheme() {
            this.setTheme(this.darkMode ^ true);
        },
        setTheme(darkMode) {
            this.darkMode = darkMode;
            document.documentElement.setAttribute("data-theme", darkMode ? "dark" : "light");
        },
    },
    mounted() {
        this.setTheme(DarkModeQuery.matches);
    },
}).mount("#quranize-header");
