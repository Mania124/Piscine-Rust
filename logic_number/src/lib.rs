pub fn number_logic(num: u32) -> bool {
    let num_str = num.to_string();
    let num_digits = num_str.len() as u32;
    let mut sum: u32 = 0;

    for digit_char in num_str.chars() {
        if let Some(digit) = digit_char.to_digit(10) {
            sum += digit.pow(num_digits);
        } else {
            // Handle potential non-digit characters (shouldn't occur with u32 input)
            return false;
        }
    }

    sum == num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_9() {
        assert_eq!(number_logic(9), true);
    }

    #[test]
    fn test_10() {
        assert_eq!(number_logic(10), false);
    }

    #[test]
    fn test_153() {
        assert_eq!(number_logic(153), true);
    }

    #[test]
    fn test_154() {
        assert_eq!(number_logic(154), false);
    }

    #[test]
    fn test_370() {
        assert_eq!(number_logic(370), true);
    }

    #[test]
    fn test_371() {
        assert_eq!(number_logic(371), true);
    }

    #[test]
    fn test_407() {
        assert_eq!(number_logic(407), true);
    }

    #[test]
    fn test_1634() {
        assert_eq!(number_logic(1634), true);
    }

    #[test]
    fn test_8208() {
        assert_eq!(number_logic(8208), true);
    }

    #[test]
    fn test_9474() {
        assert_eq!(number_logic(9474), true);
    }
}