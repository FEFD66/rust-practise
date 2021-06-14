use std::u32;
use super::snake;

pub struct Scene {
    height:u32,
    width:u32,
}

impl Scene {
    pub fn new()->Self{
        Scene{height:10,width:80}
    }
    pub fn with_size(height:u32,width:u32)->Self{
        Scene{height,width}
    }
    pub fn 
}