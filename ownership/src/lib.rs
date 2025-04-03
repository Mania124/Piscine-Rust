pub fn first_subword(mut s: String) -> String {
    let as_bytes = s.as_bytes();
    for (i, &item) in as_bytes.iter().enumerate().skip(1) {
        if item == b' ' || item == b'_' || item.is_ascii_uppercase() {
            s.truncate(i);
            break;
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s1 = "helloWorld";
        let result = first_subword(s1.to_owned());
        assert_eq!(result, "hello".to_string());
    }
}
