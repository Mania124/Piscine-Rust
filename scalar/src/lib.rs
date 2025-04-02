pub fn sum(a: u8, b: u8) -> u8 {
    a.checked_add(b).expect("ERROR: attempt to add with overflow")
}

pub fn diff(a: i16, b: i16) -> i16 {
    a.checked_sub(b).expect("ERROR: attempt to subtract with overflow")
}

pub fn pro(a: i8, b: i8) -> i8 {
    a.checked_mul(b).expect("ERROR: attempt to multiply with overflow")
}

pub fn quo(a: i32, b: i32) -> i32 {
    assert!(b != 0, "ERROR: attempt to divide by zero");
    a.checked_div(b).expect("ERROR: attempt to divide with overflow")
}

pub fn rem(a: i32, b: i32) -> i32 {
    assert!(b != 0, "ERROR: attempt to divide by zero");
    a.checked_rem(b).expect("ERROR: attempt to calculate remainder with overflow")
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
