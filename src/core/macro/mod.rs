
mod declarative;
#[cfg(test)]
mod tests{
    use codegen::simple;
    #[test]
    fn function_like_macro(){
        assert_eq!(simple!(1),1);
    }
}