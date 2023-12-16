pub fn hash(s: &str) -> u8 {
    s.chars()
        .map(|c| c as usize)
        .fold(0, |acc, item| ((acc + item) * 17) % 256) as u8
}
