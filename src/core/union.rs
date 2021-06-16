union Keycode {
    ascii:u8,
    unicode:u16,
}
pub struct C(pub u8);
#[test]
fn union(){
    let code=Keycode{ascii:12 };
    let  c=C(0);
    unsafe {
        println!("{}",code.unicode);
    }
}