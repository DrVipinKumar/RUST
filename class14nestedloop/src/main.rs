fn main() {
    let mut j=0;
    for mut i in 0..5 {
        j=0;
        while j<5 {
            print!("i ={}, j={} ",i,j);
            j+=1;
        }
        i=i+10;
        println!("i={i}");
    }
}
