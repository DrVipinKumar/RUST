fn main() {
    let mut num1=10;
    let mut num2=20;
    println!("num1={}, num2={}",num1,num2);
    swapbyref(&mut num1, &mut num2);
    println!("num1={}, num2={}",num1,num2);

}
fn swapbyref(num1: &mut i32,num2: &mut i32){
  *num1=*num1+*num2;
  *num2=*num1-*num2;
  *num1=*num1-*num2;
  println!("num1={}, num2={}",num1,num2);
}
fn swapbyvalue(mut num1: i32,mut num2: i32){
  num1=num1+num2;
  num2=num1-num2;
  num1=num1-num2;
  println!("num1={}, num2={}",num1,num2);
}
