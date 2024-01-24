use std::ops::Mul;
use std::fmt::Display;
fn main(){
    let num1=34.5;
    let num2=45.7;
    cal(num1, num2);
}
fn cal<T:Mul<Output=T>+Display>(num1:T, num2:T){
    let result=num1*num2;
    println!("Result={}",result);
}