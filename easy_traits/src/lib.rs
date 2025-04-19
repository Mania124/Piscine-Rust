#[derive(Clone)]
pub struct StringValue {
    pub value: String,
}

pub trait AppendStr {
    fn append_str(&mut self, str_to_append: String) -> Self;

    fn append_number(&mut self, nb_to_append: f64) -> Self;

    fn remove_punctuation_marks(&mut self) -> Self;
}

impl AppendStr for StringValue {
    fn append_str(&mut self, str_to_append: String) -> Self{
    
            self.value.push_str(&str_to_append);
            self.clone()
        
    }

    fn append_number(&mut self, nb_to_append: f64) -> Self{
        
            self.value.push_str(&nb_to_append.to_string());
            self.clone()
        
    }

    fn remove_punctuation_marks(&mut self) -> Self{
        
           self.value = self.value.chars()
                .filter(|cha| !matches!(cha,'.'|','|'?'|'!'))
                .collect();
            self.clone()
        
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut sv = StringValue {
            value: "Hello, world!".to_string(),
        };
    
        let sv = sv
            .append_str(" Rust?".to_string())
            .append_number(2025.0)
            .remove_punctuation_marks();
        assert_eq!(sv.value, "Hello world Rust2025".to_string());
    }
}
