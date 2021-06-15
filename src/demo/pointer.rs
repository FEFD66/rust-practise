
pub fn something(x:*mut u32){
    unsafe {
        (*x)=2;
    }
}
#[cfg(test)]
mod tests{
    use crate::demo::features::feature_test;
    use crate::demo::pointer::something;

    #[test]
    fn pointer(){
        let mut x:u32=1;
        something(&mut x);
        print!("{}",x)
    }
}