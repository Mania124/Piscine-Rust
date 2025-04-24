#[derive(Debug)]
pub struct Person{
	pub name: &str,
	pub age: u8,
}

impl Person <'a>{
	pub fn new(name: &'a str) -> Person {
        Person{
            name: name,
            age : 0,
        }
	}
}