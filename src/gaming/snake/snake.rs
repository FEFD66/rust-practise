use std::collections::LinkedList;

use crate::gaming::render::Point;

#[derive(PartialEq, Eq)]
pub enum Direction {
    Up=0,Down,Left,Right
}
impl Direction {
    pub fn get_opposite(&self)->Self{
        match self {
            Self::Up=>Self::Down,
            Self::Down=>Self::Up,
            Self::Right=>Self::Left,
            Self::Left=>Self::Right,
        }
    }
}

pub struct  Snake{
    direction:Direction,
    body:LinkedList<Point>,
}

impl Snake {
    pub fn new()->Self{
        let body=LinkedList::new();
        body.push_back((1,1));
        Snake{direction:Direction::Right,body}
    }
    pub fn move(& mut self)->bool{
        let head=se;
        self.body.push_back((0,0));
        true
    }
    pub fn changeDirection(&mut self,d:Direction){
        if self.direction.get_opposite()==d {
            return;
        }
        self.direction=d;
    }
}