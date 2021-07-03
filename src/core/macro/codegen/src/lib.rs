extern crate proc_macro;
use proc_macro::TokenStream;

mod procedural;
#[proc_macro]
pub fn simple(input:TokenStream)->TokenStream{
    return input;
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
