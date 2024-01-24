use std::io;
fn main() {
    let num1:f32;
    let num2:f32;   
    println!("Enter number 1=>");
    num1=get_input();
    println!("Enter number 2=>");
    num2=get_input();
    println!("Sum of two number is ={}",(num1+num2));
    // println!("Enter your name=>");
    // io::stdin().read_line(&mut value).unwrap();
    // println!("{}",value);
}
fn get_input()-> f32 {
    let mut value:String=String::new();
    io::stdin().read_line(&mut value).unwrap();
    return value.trim().parse::<f32>().unwrap();
}
