
pub fn something(x:*mut u32){
    unsafe {
        (*x)=2;
    }
}
#[cfg(test)]
mod tests{
    use crate::core::pointer::something;

    #[test]
    fn pointer(){
        let mut x:u32=1;
        something(&mut x);
        print!("{}",x)
    }
}