pub fn num_to_ordinal(x: u32) -> String {
    let x_str = x.to_string();
    let last_two_digits = x % 100;
    let last_digit = x % 10;

    if 11 <= last_two_digits && last_two_digits <= 13 {
        format!("{}th", x_str)
    } else {
        match last_digit {
            1 => format!("{}st", x_str),
            2 => format!("{}nd", x_str),
            3 => format!("{}rd", x_str),
            _ => format!("{}th", x_str),
        }
    }
}
