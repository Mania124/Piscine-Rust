pub fn is_empty(v: &str) -> bool {
    v.chars().count()==0
}

pub fn is_ascii(v: &str) -> bool {
    v.is_ascii()
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {
    v.chars().position(|c| c == pat).unwrap_or(v.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result1 = is_empty("");
        assert_eq!(result1, true);
        let result2 = is_ascii("rust");
        assert_eq!(result2, true);
        let result3 = contains("rust", "ru");
        assert_eq!(result3, true);
        let result4 = split_at("rust", 2);
        assert_eq!(result4, ("ru", "st"));
        let result5 = find("rust", 'u');
        assert_eq!(result5, 1);
    }
}
