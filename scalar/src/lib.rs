pub fn sum(a: u8, b: u8) -> u8 {
    a.checked_add(b).expect("ERROR: attempt to add with overflow")
}

pub fn diff(a: i16, b: i16) -> i16 {
    a.checked_sub(b).expect("ERROR: attempt to subtract with overflow")
}

pub fn pro(a: i8, b: i8) -> i8 {
    a.checked_mul(b).expect("ERROR: attempt to multiply with overflow")
}

pub fn quo(a: f32, b: f32) -> f32 {
    assert!(b != 0.0, "ERROR: attempt to divide by zero");
    a/b
}

pub fn rem(a: f32, b: f32) -> f32 {
    assert!(b != 0.0, "ERROR: attempt to divide by zero");
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
        let result = quo(2.0, 2.0);
        assert_eq!(result, 1.0);
        let result = rem(2.0, 2.0);
        assert_eq!(result, 0.0);
    }
}
