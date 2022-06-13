use std::pin::Pin;

mod filemanager;

trait File{
    fn poll(self:Pin<&mut Self>)->u32;
}

struct A{
}

fn main() {
    println!("Hello, world!");
}
fn a(a:&mut String){
    let x=b(a);
    *a=String::from("213");
    println!("{}",x);
}

fn b(a:& mut String)->&mut u8{
    cout.operator<<()
}
