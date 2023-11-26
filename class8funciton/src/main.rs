fn main() {
   let num1:i32=30;
   let num2:i32=20;
   display_msg();
   sum(num1,num2);
   println!("Subtraction of two number is ={}",sub(num1,num2));
   println!("Multiplication of two number is ={}",mul(num1,num2));
   println!("Addition and Multiplication of two number is ={:?}",cal(num1,num2));
}
fn display_msg(){  //simple function with no parameter and no return value
    println!("Message by Display Functions!");
}
fn sum(num1:i32,num2:i32){ //function with parameter and no return value
    println!("Sum of two number is ={}",(num1+num2));
}
fn sub(num1:i32,num2:i32)-> i32 { //function with parameter and return value
    return num1-num2;
}
fn mul(num1:i32, num2:i32)-> i32 { //function with patemeter and return value by expression
    num1*num2        // expression
}
fn cal(num1:i32,num2:i32)-> (i32,i32) {  //fuction with return multiple values
    (num1+num2,num1*num2)
}
// function is name of a block that use to perform specific
// task and it can return value after completing task, not 
// only that we can also pass parameter to function, and 
// it can be call or use any number of times. 
