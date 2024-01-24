use std::ops::Mul;
use std::marker::Copy;
use std::fmt::Display;
struct Rectangle<T>
{
    width:T,
    height:T,
}
impl <T:Mul<Output=T> + Copy + Display> Rectangle<T>
{
    fn get_area(&self) {
       let result=self.width*self.height;
       println!("Area of Rectangle={}",result);
    }
}
fn main() {
    let r1 =Rectangle{width:34,height:45};
    r1.get_area();
}
