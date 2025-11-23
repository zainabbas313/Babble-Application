#[derive(Debug)]
pub struct User {
    pub name: String,
    pub position: String,
    pub age: u32,
}

impl User{
    pub fn new(name: &str, position: &str, age: u32) -> Self {
         Self {
            name: name.to_string(),
            position: position.to_string(),
            age: age
         }
    }

    pub fn greet(&self) {
        println!("Hello, my name is {} and I'm {} years old. I am currently a {}", self.name, self.age, self.position)
    }
}
