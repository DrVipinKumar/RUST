enum Check {True=40,False=50}
const PI:f32=3.14;
fn main() {
    let info:Check =Check::False;
    match info {
        Check::True => println!("It is True"),     
        Check::False => println!("It is False"),
    }
    println!("PI={}",PI);
}
