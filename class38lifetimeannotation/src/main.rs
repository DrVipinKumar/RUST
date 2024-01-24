fn main() {
    let str1=String::from("lifetime annotation");
    let gstr;
    {
    let str2=String::from("Multiple lifetime annotation");  
    gstr=get_greater_string(&str1,&str2);    
    }  
    println!("Greater Length String is ={}",gstr);
}
fn get_greater_string<'a,'b>(str1:&'a str,str2:&'b str)->&'a str {
    if str1.len()>str2.len() {
        str1
    } else {
        str1
    }
}