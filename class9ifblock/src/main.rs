fn main() {
    let x:i32=20;
    if false {
        println!("value of x is greater than or equal to 10");
    } else 
    {
        println!("value of x is less than 10");
    }
    let result=if x>10 {
        println!("return by true condition");
        x+100
    } else {
        x+200
    };
    println!("Value of result is ={result}");
    
}
