pub fn is_pangram(s: &str) -> bool {
    let mut alphabet = [false; 26];
    for c in s.chars() {
        if c.is_alphabetic() {
            let index = c.to_lowercase().next().unwrap() as usize - 'a' as usize;
            if index < 26 {
                alphabet[index] = true;
            }
        }
    }
    alphabet.iter().all(|&present| present)
}
