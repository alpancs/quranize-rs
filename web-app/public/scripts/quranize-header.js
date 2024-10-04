import { createApp } from "./vue.esm-browser.js";

createApp({
    data: () => ({ darkMode: undefined }),
    methods: {
        toggleTheme() {
            this.setDataTheme(this.darkMode ? "light" : "dark");
        },
        setDataTheme(dataTheme) {
            document.documentElement.setAttribute("data-theme", dataTheme);
            this.darkMode = dataTheme === "dark";
            sessionStorage.dataTheme = dataTheme;
        },
    },
    mounted() {
        const dataTheme = sessionStorage.dataTheme ||
            (matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light");
        this.setDataTheme(dataTheme);
    },
}).mount("#quranize-header");
