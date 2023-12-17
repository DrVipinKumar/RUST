fn main() {
    for i in 10..20 {  //for with range
        println!("Value of i ={}",i);
    }
    let data=[12,34,56,78,90];
    for (index,value) in data.iter().enumerate() {
        println!("Value of Array[{}] ={}",index,value);
    }
}
