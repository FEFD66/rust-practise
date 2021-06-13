pub mod features;

#[cfg(test)]
mod tests{
    use crate::demo::features::feature_test;

    #[test]
    fn features(){
        feature_test();
    }
}