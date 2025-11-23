mod user;

use std::collections::HashMap;

fn add(x: i8, y: i8) -> i8 {
    x + y
}

fn play_with_array() {
    let mut arr1: [i32; 3] = [1, 2, 3];
    let arr2: [&str; 4] = ["London", "Manchester", "Karachi", "Dubai"];

    // array of 3 tuples (arr2, arr1)
    arr1[0] = 90;
    let arr3 = [(arr2, arr1); 3];
    println!("{:?}", arr1);
    println!("{:?}", arr2);
    println!("{:#?}", arr3);
}

fn dict() {
    // specify type <&str, &str>
    let mut dict: HashMap<&str, &str> = HashMap::new();

    dict.insert("name", "zain");
    dict.insert("age", "24");

    println!("{:?}", dict);
}

fn dict_by_enum(){
    #[allow(dead_code)]
    #[derive(Debug)]
    enum Value{
        Text(String),
        Number(i32),
        Float(f64),
        // Bool(bool)
    }

    let mut dict: HashMap<String, Value> = HashMap::new();
    dict.insert("name".to_string(), Value::Text("Zain".to_string()));
    dict.insert("age".to_string(), Value::Number(24));
    dict.insert("Role in Google".to_string(), Value::Text("Senior AI Software Architect".to_string()));
    dict.insert("Salary".to_string(), Value::Float(25035.80));
    println!("{:#?}", dict)
}

fn main() {
    println!("Hello, world!");
    println!("{}", add(5, 5));
    
    play_with_array();
    dict();
    dict_by_enum();
    use user::User;
    let obj = User::new("Zain", "Senior AI SWE Architech - Google", 24);
    println!("{:#?}", obj);
    obj.greet()
}
