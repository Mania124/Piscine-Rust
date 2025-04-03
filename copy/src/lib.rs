pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let new = c.abs() as f64;
    let exponential_value = (c as f64).exp();
    let natural_log = if new == 0.0 {
        f64::NEG_INFINITY
    } else {
        new.ln()
    };
    (c, exponential_value, natural_log)
}

pub fn str_function(a: String) -> (String, String) {
    let exp_values = a
        .split_whitespace()
        .filter_map(|num| num.parse::<f64>().ok())
        .map(|x| x.exp().to_string())
        .collect::<Vec<_>>()
        .join(" ");

    (a, exp_values)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let abs_log_values: Vec<f64> = b.iter().map(|&x| (x.abs() as f64).ln()).collect();
    (b, abs_log_values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = "1 2 4 5 6".to_owned();
        let b = vec![1, 2, 4, 5];
        let result1 = str_function(a);
        assert_eq!(result1,("1 2 4 5 6".to_string(), "2.718281828459045 7.38905609893065 54.598150033144236 148.4131591025766 403.4287934927351".to_string()));
        let result2 = vec_function(b);
        assert_eq!(
            result2,
            (
                vec![1, 2, 4, 5],
                vec![
                    0.0,
                    0.6931471805599453,
                    1.3862943611198906,
                    1.6094379124341003
                ]
            )
        );
    }
}
