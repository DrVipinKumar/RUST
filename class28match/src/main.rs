fn main() {
    let value:Option<i32>=Default::default();
    match value {
        Some(v) => println!("It is {}",v),
        None =>println!("None this time"),
    }
    let check:i32=1;
    match check {
        1 => println!("It is 1"),
        2 => println!("It is 2"),
        _ => println!("It does not match"),        
    }
    
}
