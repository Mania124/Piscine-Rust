pub fn divide(x: i32, y: i32) -> (i32, i32) {
    if y!=0{
        let div=x/y;
        let rem = x%y;
        (div, rem)
    }else{
        panic!("Division by zero!");
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = divide(10, 3);
        assert_eq!(result, (3, 1));
    }
}
