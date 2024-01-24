use std::env;
fn main() {
   let argument:Vec<String>=env::args().collect();
//    for arg in &argument{
//     println!("{}",arg);
  let num1:i32;
  let num2:i32;
  num1=argument[1].trim().parse().unwrap();
  num2=argument[2].trim().parse().unwrap();
  println!("Sum of two number by cmd argument={}",(num1+num2))

}
