pub fn rev_str(input: &str) -> String {
    input.chars().rev().collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = rev_str("2, 2");
        assert_eq!(result, "2 ,2");
    }
}
