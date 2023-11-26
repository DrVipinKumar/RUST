fn main() {
    let mut i=0;
    let result=loop {
        if i>10 {
            break i*10;
        }
        println!("I am in Loop {i}");
        i=i+1;
    };
    println!("result is {result}");
}
