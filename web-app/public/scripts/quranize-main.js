import EventStatus from "./event-status.js";
import { createApp } from "./vue.esm-browser.js";
import { suraNames } from "./quran/meta.js";

const quranizeWorker = new Worker("scripts/web-worker.js", { type: "module" });

const app = createApp({
    data() {
        return {
            isEngineReady: false,
            keyword: "",
            encodeResults: [],
            supportSharing: "share" in navigator,
            examples: getExamples(),
            translations: { EN: { path: "./quran/en.sahih.js" }, ID: { path: "./quran/id.indonesian.js" } },
        };
    },
    computed: {
        hasResults() { return this.encodeResults.length > 0; },
        hasEmptyResult() { return this.isEngineReady && this.keyword !== "" && !this.hasResults; },
    },
    methods: {
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
        isArActive: location => location.activeTab === undefined || location.activeTab === 'AR',
        ayaSuffix: location => `${suraNames[location.sura_number - 1]}:${toArabicNumber(location.aya_number)}`,
        tanzilURL: location => `https://tanzil.net/#${location.sura_number}:${location.aya_number}`,
        navigateTab(location, tab) {
            location.activeTab = tab;
            if (tab === 'ID' || tab === 'EN') {
                delete location.translation;
                const index = `${location.sura_number}:${location.aya_number}`;
                this.getTranslation(tab).then(map => location.translation = map[index]);
            }
        },
        async getTranslation(translation) {
            if (!this.translations[translation]) return {};
            if (this.translations[translation].map) return this.translations[translation].map;
            let map = {};
            (await import(this.translations[translation].path)).default
                .split("\n")
                .map(l => l.split("|"))
                .filter(x => x.length === 3)
                .forEach(x => map[`${x[0]}:${x[1]}`] = x[2]);
            this.translations[translation].map = map;
            return map;
        },
        share() {
            navigator.share({ url: `${location.href}#${encodeURIComponent(this.keyword.trim())}` });
        },
    },
    mounted() {
        const URLHash = location.hash.replace(/^#/, "");
        if (URLHash) {
            this.setKeyword(decodeURIComponent(URLHash));
            history.pushState({}, "", location.href.replace(/#.*$/, ""));
        }
        this.$refs.keyword.focus();
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

if ("serviceWorker" in navigator) navigator.serviceWorker.register("service-worker.js");

function getExamples() {
    let candidates = [
        "bismillah", "subhanallah", "alhamdulillah", "masyaallah", "insyaallah", "inna lillahi wainna ilaihi roji'un",
        "waantum muslimun", "ya ayyuhannas", "walaqod yassarna", "waltandur nafs", "tabaarokalladzi", "wabarron biwalidati",
    ];
    let examples = [];
    const EXAMPLE_COUNT = 5 + Math.floor(Math.random() * 3);
    for (let i = 0; i < EXAMPLE_COUNT; i++)
        examples.push(...candidates.splice(Math.floor(Math.random() * candidates.length), 1));
    return examples;
}

function toArabicNumber(n) {
    if (n < 0) return `-${toArabicNumber(-n)}`;
    if (n < 10) return String.fromCharCode(0x0660 + n);
    return toArabicNumber(Math.floor(n / 10)) + toArabicNumber(n % 10);
}
