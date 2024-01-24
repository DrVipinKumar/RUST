struct Student {
    sid:i32,
    name:String,
}
fn main() {
    let s1=Student{sid:1, name:String::from("Dr. Vipin")};
    call_struct(&s1);
    println!("Student ID={}, Student Name={}",s1.sid,s1.name);
}
fn call_struct(s1:&Student){
    println!("Student ID={}, Student Name={}",s1.sid,s1.name);
}
