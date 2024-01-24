fn main() {
    // let mut x=10;
    // println!("x = {}",x);
    // let y=&mut x;
    // *y=*y+10;
    // println!("y = {}",y);
    // //println!("x = {}",x);
    // println!("y = {}",y);
    let name:String=String::from("Dr. Vipin Classes");
    display_Message(&name);
    display_Message(&name);
}
fn display_Message(name:&String){
    println!("{}",name);
}
