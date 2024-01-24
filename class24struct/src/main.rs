struct Student { //struct
    name:String,
    sid:i32,
}
struct Emp(String,i32,f32);//tuple struct
fn main() {
    let s1=Student{name:String::from("Dr. Vipin Classes"),sid:1};
    let s2=Student{name:String::from("Dr. Vipin Teotia"),sid:2};
    let e1 =Emp(String::from("Dr. Vipin"),1,20000.233);
    println!("Student ID={}, Student Name={}",s1.sid,s1.name);
    println!("Student ID={}, Student Name={}",s2.sid,s2.name);
    println!("Employee ID={}, Employee Name={}, Employee Salary={}",e1.1,e1.0,e1.2);
    
}
