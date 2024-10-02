import EventStatus from "./event-status.js";
import { createApp } from "./vue.esm-browser.js";
import { suraNames } from "./quran/meta.js";

const quranizeWorker = new Worker("scripts/web-worker.js", { type: "module" });

const app = createApp({
    mounted() {
        this.registerServiceWorker();
        this.captureURLHash();
        this.focusInInput();
    },

    data() {
        return {
            isEngineReady: false,
            keyword: "",
            encodeResults: [],
            supportSharing: "share" in navigator,
            examples: getExamples(),
            translations: {
                EN: { URL: "scripts/quran/en.sahih.txt" },
                ID: { URL: "scripts/quran/id.indonesian.txt" },
            },
            playing: "",
        };
    },

    computed: {
        hasResults() { return this.encodeResults.length > 0; },
        hasEmptyResult() { return this.isEngineReady && this.keyword !== "" && !this.hasResults; },
    },
    methods: {
        registerServiceWorker() {
            navigator.serviceWorker?.register("service-worker.js");
        },
        captureURLHash() {
            const urlHash = location.hash.replace(/^#/, "");
            if (urlHash) {
                this.setKeyword(decodeURIComponent(urlHash));
                history.pushState({}, "", location.href.replace(/#.*$/, ""));
            }
        },
        focusInInput() {
            setTimeout(() => { this.$refs.keyword.focus(); }, 530);
        },

        keywordInputted(event) {
            this.setKeyword(event.target.value);
        },
        deleteKeyword() {
            this.setKeyword("");
            this.$refs.keyword.focus();
        },
        setKeyword(keyword) {
            this.keyword = keyword;
            quranizeWorker.postMessage({ status: EventStatus.KeywordUpdated, keyword });
        },
        clickExpand(result) {
            if (!result.compactExpls || !result.locations) quranizeWorker.postMessage({
                status: EventStatus.ResultClicked, quran: result.quran, expl: result.explanation
            });
            result.expanding ^= true;
        },
        suraName: location => suraNames[location.sura_number - 1],
        ayaNumber: location => toArabicNumber(location.aya_number),
        tanzilURL: location => `https://tanzil.net/#${location.sura_number}:${location.aya_number}`,
        audioSource: location => `https://tanzil.net/res/audio/matrood/${location.sura_number.toString().padStart(3, "0")}${location.aya_number.toString().padStart(3, "0")}.mp3`,
        async navigateTranslation(location, translation) {
            if (location.activeTranslation === translation) {
                delete location.activeTranslation;
                delete location.translation;
                return;
            }

            location.activeTranslation = translation;
            if (this.translations[translation]?.data) {
                location.translation = this.translations[translation].data[location.index];
            } else {
                delete location.translation;
                const url = this.translations[translation]?.URL;
                const data = (await (await fetch(url)).text()).split("\n");
                this.translations[translation].data = data;
                if (location.activeTranslation === translation) {
                    location.translation = data[location.index];
                }
            }
        },
        togglePlay(result, location) {
            const id = result.quran + location.index;
            this.playing = this.playing === id ? '' : id;
        },
        isPlaying(result, location) {
            return this.playing === result.quran + location.index;
        },
        share() {
            navigator.share({ url: `${location.href}#${encodeURIComponent(this.keyword.trim())}` });
        },
        copyToClipboard: text => navigator.clipboard?.writeText(text),
    },
}).mount("#quranize-main");

quranizeWorker.onmessage = event => {
    const message = event.data;
    switch (message.status) {
        case EventStatus.EngineInitiationStarted:
            app.isEngineReady = false;
            break;
        case EventStatus.EngineInitiated:
            app.isEngineReady = true;
            break;
        case EventStatus.KeywordEncoded:
            if (message.keyword === app.keyword) app.encodeResults = message.encodeResults;
            break;
        case EventStatus.ResultLocated:
            const result = app.encodeResults.find(result => result.quran === message.quran);
            if (result) {
                result.compactExpls = message.compactExpls;
                result.locations = message.locations;
            }
            break;
    }
};

function getExamples() {
    let candidates = [
        "bismillah", "subhanallah", "alhamdulillah", "masyaallah", "insyaallah", "inna lillahi wainna ilaihi roji'un",
        "waantum muslimun", "ya ayyuhannas", "walaqod yassarna", "waltandur nafs", "tabaarokalladzi", "wabarron biwalidati",
    ];
    let examples = [];
    const EXAMPLE_COUNT = 5 + Math.trunc(Math.random() * 3);
    for (let i = 0; i < EXAMPLE_COUNT; i++)
        examples.push(...candidates.splice(Math.trunc(Math.random() * candidates.length), 1));
    return examples;
}

function toArabicNumber(n) {
    if (n < 0) return `-${toArabicNumber(-n)}`;
    if (n < 10) return String.fromCharCode(0x0660 + n);
    return toArabicNumber(Math.trunc(n / 10)) + toArabicNumber(n % 10);
}