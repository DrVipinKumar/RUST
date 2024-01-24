fn main() {
    let mut data:Vec<i32>=Vec::new();
    data.push(32);
    data.push(40);
    data.remove(0);
    let mut info:Vec<i32>=vec![2,3,4,5,67];
    info.push(100);
    info.remove(3);
    println!("Data Vector Values={:?}",data);
    println!("Info Vector Values={:?}",info);
    match info.pop() {
        Some(infov)=>println!("After pop={:?}",infov),
        None => println!("No value found in info vector"),
    }
    println!("Info Vector Values={:?}",info);
}
