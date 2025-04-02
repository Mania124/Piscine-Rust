pub fn sum(a: u8, b: u8) -> u8{
    a+b
}

pub fn diff(a: i16, b: i16) -> i16 {
    a-b
}

pub fn pro(a: i8, b: i8) -> i8 {
    a*b
}

pub fn quo(a: i32, b: i32) -> i32 {
    a/b
}

pub fn rem(a: i32, b: i32) -> i32 {
    a%b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = sum(2, 2);
        assert_eq!(result, 4);
        let result = diff(2, 2);
        assert_eq!(result, 0);
        let result = pro(2, 2);
        assert_eq!(result, 4);
        let result = quo(2, 2);
        assert_eq!(result, 1);
        let result = rem(2, 2);
        assert_eq!(result, 0);
    }
}
