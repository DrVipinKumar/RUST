struct Rectangle<T,U> {
    length:T,
    breath:U,
}
fn main() {
    let rec1 =Rectangle{length:4,breath:6.7};
    println!("length={},and Breath={}",rec1.length,rec1.breath);
    let rec2 =Rectangle{length:4.4,breath:String::from("23")};
    println!("length={},and Breath={}",rec2.length,rec2.breath);
}
