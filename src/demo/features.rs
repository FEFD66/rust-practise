
#[cfg(feature = "user_define_feature")]
pub fn feature_test(){
    println!("value1 works!")
}

//互斥feature
#[cfg(all(feature = "conflict1",feature = "conflict2"))]
compile_error!("feature conflict1 and feature conflict2 connot be enabled at the same time.");

#[cfg(not(feature = "user_define_feature"))]
pub fn feature_test(){
    println!("uer_define_feature not work!")
}
