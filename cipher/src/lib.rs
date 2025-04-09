#[derive(Debug, PartialEq)]
pub struct CipherError {
    // expected public fields
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let expected: String = original
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                if c.is_ascii_lowercase() {
                    (b'z' - (c as u8 - b'a')) as char
                } else {
                    (b'Z' - (c as u8 - b'A')) as char
                }
            } else {
                c
            }
        })
        .collect();
    if expected == ciphered {
        Ok(())
    } else {
        Err(CipherError { expected })
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = cipher("1Hello 2world!", "1Svool 2dliow!");
        assert_eq!(result, Ok(()));
        let res = cipher("1Hello 2world!", "svool");
        assert_eq!(
            res,
            Err(CipherError {
                expected: "1Svool 2dliow!".to_string()
            })
        );
    }
}
