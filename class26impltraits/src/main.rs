struct Message {
    msg:String,
}
trait MyMsg {
    fn get_mymsg(&self);
}
impl MyMsg for Message {
    fn get_mymsg(&self) {
        println!("Message by Traits={}",self.msg);
    }
}
impl Message {
    fn display_msg(&self){
        println!("Message={}",self.msg);
    }
    fn another_msg(&self){
        println!("This is another message in Structure");
    }
}
fn main() {
    let msg1:Message =Message{msg:String::from("Welcome to Dr. Vipin Classes to learn Rust")};
    msg1.display_msg();
    msg1.another_msg();
    msg1.get_mymsg()
}
