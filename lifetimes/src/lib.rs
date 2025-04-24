#[derive(Debug)]
pub struct Person<'a>{
	pub name: &'a str,
	pub age: u8,
}

impl <'a> Person <'a>{
	pub fn new<'a>(name: &'a str) -> Person {
        Person{
            name: name,
            age : 0,
        }
	}
}