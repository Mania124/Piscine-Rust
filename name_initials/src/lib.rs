pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
    .into_iter()
    .map(|n| {
        let parts: Vec<&str> = n.split_whitespace().collect(); // Split by spaces

        match parts.as_slice() {
            [first, last, ..] => format!("{}. {}.", first.chars().next().unwrap(), last.chars().next().unwrap()),
            [first] => format!("{}.", first.chars().next().unwrap()), // Handle single names
            _ => String::new(), // Empty string for unexpected cases (like empty input)
        }
    })
    .collect()
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = initials(vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"]);
        assert_eq!(result, ["H. P.", "S. E.", "J. L.", "B. O."]);
    }
}
