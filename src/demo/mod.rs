
#[cfg(test)]
 mod features;
mod pointer;
#[cfg(test)]
mod lifetime;
#[cfg(test)]
mod union;

#[cfg(test)]
mod tests{
    use crate::demo::features::feature_test;

    #[test]
    fn features(){
        feature_test();
    }
}