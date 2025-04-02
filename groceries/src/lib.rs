pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val);
}

pub fn at_index(slice: &[String], index: usize) -> &str {
    if index < slice.len() {
        &slice[index]
    } else {
        panic!("Index out of bounds")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut groceries = vec![
            "yogurt".to_string(),
            "panettone".to_string(),
            "bread".to_string(),
            "cheese".to_string(),
        ];
        insert(&mut groceries, String::from("nuts"));
        assert_eq!(
            groceries,
            vec![
                "yogurt".to_string(),
                "panettone".to_string(),
                "bread".to_string(),
                "cheese".to_string(),
                "nuts".to_string(),
            ]
        );
        let res = at_index(&groceries, 1);
        assert_eq!(res, "panettone");
    }
}
