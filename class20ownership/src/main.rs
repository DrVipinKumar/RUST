fn main() {
    let num=10;   //scalar data type  stack/heap
    println!("Num={}",num);
    let mut num1=num;
    num1=20;
    println!("Num1={}",num1);
    println!("Num={}",num);

//     let mut name:String=String::from("Dr. Vipin Classes");
//     println!("Name={}",name);
//    // let name1=name;  //Move Ownership
//     let mut name1=name.clone(); //Clone Ownership
//     name1=String::from("Dr. Vipin Kumar");
//     println!("Name1={}",name1);
//     name=String::from("Dr. Vipin Teotia");
//     println!("Name={}",name); //name does not have access to data it store
}
