use std::collections::HashMap;
fn main() {
    let mut smarks:HashMap<String, i32>=HashMap::new();
    smarks.insert("Rahul".to_string(), 90);
    smarks.insert("Amit".to_string(), 80);
    smarks.insert("Dr. Vipin".to_string(), 95);
    smarks.remove("Amit");
    for (name,mark) in &smarks {
        println!("Student Name={}, Marks={}",name, mark);
    }
    println!("is name exits ={}",smarks.contains_key("Amit Kumar"));
}
