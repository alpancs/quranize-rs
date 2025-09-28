import { ref } from 'vue';
import Worker from '../workers/quranize.ts?worker';
import type { Quranize } from './quranize-wasm';

export type PageItem = {
    sura: number;
    aya: number;
    text: string;
};

export const initiated = ref(false);

const worker = new Worker();
const resolves = new Map<number, Function>();

worker.onmessage = ({ data: { id, resp } }) => {
    if (id === 0) {
        initiated.value = true
    } else {
        resolves.get(id)?.(resp)
        resolves.delete(id)
    }
};

let counter = 0;
export function call<T>(func: keyof Quranize, ...args: any[]) {
    const id = ++counter;
    const promise = new Promise<T>((resolve) => resolves.set(id, resolve));
    worker.postMessage({ id, func, args });
    return promise;
}

export function toArabicNumber(n: number): string {
    if (n === undefined) return "";
    if (n < 0) return `-${toArabicNumber(-n)}`;
    if (n < 10) return String.fromCharCode(0x0660 + n);
    return toArabicNumber(Math.trunc(n / 10)) + toArabicNumber(n % 10);
}

const SuraNamesAR = ["الفاتحة", "البقرة", "آل عمران", "النساء", "المائدة", "الأنعام", "الأعراف", "الأنفال", "التوبة", "يونس", "هود", "يوسف", "الرعد", "ابراهيم", "الحجر", "النحل", "الإسراء", "الكهف", "مريم", "طه", "الأنبياء", "الحج", "المؤمنون", "النور", "الفرقان", "الشعراء", "النمل", "القصص", "العنكبوت", "الروم", "لقمان", "السجدة", "الأحزاب", "سبإ", "فاطر", "يس", "الصافات", "ص", "الزمر", "غافر", "فصلت", "الشورى", "الزخرف", "الدخان", "الجاثية", "الأحقاف", "محمد", "الفتح", "الحجرات", "ق", "الذاريات", "الطور", "النجم", "القمر", "الرحمن", "الواقعة", "الحديد", "المجادلة", "الحشر", "الممتحنة", "الصف", "الجمعة", "المنافقون", "التغابن", "الطلاق", "التحريم", "الملك", "القلم", "الحاقة", "المعارج", "نوح", "الجن", "المزمل", "المدثر", "القيامة", "الانسان", "المرسلات", "النبإ", "النازعات", "عبس", "التكوير", "الإنفطار", "المطففين", "الإنشقاق", "البروج", "الطارق", "الأعلى", "الغاشية", "الفجر", "البلد", "الشمس", "الليل", "الضحى", "الشرح", "التين", "العلق", "القدر", "البينة", "الزلزلة", "العاديات", "القارعة", "التكاثر", "العصر", "الهمزة", "الفيل", "قريش", "الماعون", "الكوثر", "الكافرون", "النصر", "المسد", "الإخلاص", "الفلق", "الناس"];
const SuraNamesID = ["Al-Faatiha", "Al-Baqara", "Aal-i-Imraan", "An-Nisaa", "Al-Maaida", "Al-An'aam", "Al-A'raaf", "Al-Anfaal", "At-Tawba", "Yunus", "Hud", "Yusuf", "Ar-Ra'd", "Ibrahim", "Al-Hijr", "An-Nahl", "Al-Israa", "Al-Kahf", "Maryam", "Taa-Haa", "Al-Anbiyaa", "Al-Hajj", "Al-Muminoon", "An-Noor", "Al-Furqaan", "Ash-Shu'araa", "An-Naml", "Al-Qasas", "Al-Ankaboot", "Ar-Room", "Luqman", "As-Sajda", "Al-Ahzaab", "Saba", "Faatir", "Yaseen", "As-Saaffaat", "Saad", "Az-Zumar", "Al-Ghaafir", "Fussilat", "Ash-Shura", "Az-Zukhruf", "Ad-Dukhaan", "Al-Jaathiya", "Al-Ahqaf", "Muhammad", "Al-Fath", "Al-Hujuraat", "Qaaf", "Adh-Dhaariyat", "At-Tur", "An-Najm", "Al-Qamar", "Ar-Rahmaan", "Al-Waaqia", "Al-Hadid", "Al-Mujaadila", "Al-Hashr", "Al-Mumtahana", "As-Saff", "Al-Jumu'a", "Al-Munaafiqoon", "At-Taghaabun", "At-Talaaq", "At-Tahrim", "Al-Mulk", "Al-Qalam", "Al-Haaqqa", "Al-Ma'aarij", "Nooh", "Al-Jinn", "Al-Muzzammil", "Al-Muddaththir", "Al-Qiyaama", "Al-Insaan", "Al-Mursalaat", "An-Naba", "An-Naazi'aat", "Abasa", "At-Takwir", "Al-Infitaar", "Al-Mutaffifin", "Al-Inshiqaaq", "Al-Burooj", "At-Taariq", "Al-A'laa", "Al-Ghaashiya", "Al-Fajr", "Al-Balad", "Ash-Shams", "Al-Lail", "Ad-Dhuhaa", "Ash-Sharh", "At-Tin", "Al-Alaq", "Al-Qadr", "Al-Bayyina", "Az-Zalzala", "Al-Aadiyaat", "Al-Qaari'a", "At-Takaathur", "Al-Asr", "Al-Humaza", "Al-Fil", "Quraish", "Al-Maa'un", "Al-Kawthar", "Al-Kaafiroon", "An-Nasr", "Al-Masad", "Al-Ikhlaas", "Al-Falaq", "An-Naas"];

export async function getPageItemGroups(page: number) {
    const pageItems = await call<PageItem[]>('getPage', page);
    return pageItems.reduce<PageItem[][]>((groups, curr, i) => {
        if (i === 0 || curr.sura > pageItems[i - 1]!.sura) groups.push([curr]);
        else groups[groups.length - 1]!.push(curr);
        return groups;
    }, []);
}

export function getSuraNameAR(suraNumber: number) {
    return SuraNamesAR[suraNumber - 1];
}

export function getSuraNameID(suraNumber: number) {
    return SuraNamesID[suraNumber - 1];
}
