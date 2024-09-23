pub(super) fn quran_iter(s: &str) -> impl Iterator<Item = (usize, &str)> {
    (0..6236).zip(s.split('\n'))
}
