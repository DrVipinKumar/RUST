fn main() {
    let mut name:String=String::from(" this is \nexamle of string\n data type in rust  ");
    println!("{}",name);
    name=name.replace("examle","example");
    for n in name.lines(){
        println!("Lines {}",n);
    }
    for n in name.split(" "){
        println!("split {}",n);
    }
    let name1:String=String::from("this is string   ");
    println!("{}",name1.trim());
    if let Some(c)=name1.chars().nth(3) {
        println!("{}",c);
    }

}
