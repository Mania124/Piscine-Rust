#[derive(Debug, PartialEq, Eq)]
pub struct Student(pub u32, pub String, pub String);
pub fn id(student: &Student) -> u32 {
    student.0
}

pub fn first_name(student: &Student) -> &str {
    &student.1
}

pub fn last_name(student: &Student) -> &str {
    &student.2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let student = Student(20, "Pedro".to_string(), "Domingos".to_string());
        let result1 = first_name(&student);
        assert_eq!(result1, "Pedro");
        let result2 = last_name(&student);
        assert_eq!(result2, "Domingos");
        let result3 = id(&student);
        assert_eq!(result3, 20);
    }
}
