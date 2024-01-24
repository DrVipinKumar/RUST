enum Shape {
    Circle(f64),
    Rectangle(f64,f64),
    Triangle(f64,f64,f64),
}
impl Shape {
    fn display(&self){
        match self {
            Shape::Circle(r)=>println!("Radius ={}",r),
            Shape::Rectangle(l,b )=>println!("Length={},Breath={}",l,b),
            Shape::Triangle(a,b ,c)=>println!("Side 1={}, Side 2={}, and Side 3={}",a,b,c),
        }
    }
}
fn main() {
    let sh=Shape::Triangle(4.5,56.5,4.7);
    sh.display();
}
