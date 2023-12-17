fn main() {
    let data =[12,16,34,4,67,23,200,8];
    let mut max=data[0];
    let mut min=data[0];
    for i in 1..8 {
        if max < data[i] {
            max=data[i];
        } else if min>data[i]{
            min=data[i];
        }
    }
    println!("max={}, and min={}",max, min);
}
