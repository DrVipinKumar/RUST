fn main() {
    let mut x=10;
    x=40;
    println!("X = {}",x);
    {
        let x=45.7;
        println!("X = {}",x);
    }
    let x="Dr. Vipin Classes";  //creating new variable with same name as declared earlier
    println!("X = {}",x);
}
