use std::collections::LinkedList;

use crate::gaming::render::{Point, GeneralRender, Render};
use std::borrow::Borrow;
use std::option::Option::Some;

#[derive(PartialEq, Eq,Copy, Clone)]
pub enum Direction {
    None=0,Up,Down,Left,Right
}
impl Direction {
    pub fn get_opposite(&self)->Self{
        match self {
            Self::Up=>Self::Down,
            Self::Down=>Self::Up,
            Self::Right=>Self::Left,
            Self::Left=>Self::Right,
            Self::None=>Self::None,
        }
    }
}

pub struct  Snake{
    direction:Direction,
    body:LinkedList<Point>,
    old:Option<Point>,
}

impl Snake {
    pub fn new()->Self{
        let mut body=LinkedList::new();
        body.push_back((1,1));
        body.push_back((1,2));
        body.push_back((1,3));
        Snake{direction:Direction::Right,body,old:None}
    }
    pub fn next_pos(&self)->Point{
        let first=self.body.back().unwrap();
        match self.direction {
            Direction::Up => {(first.0,first.1-1) }
            Direction::Down => {(first.0,first.1+1)}
            Direction::Left => {(first.0-1,first.1)}
            Direction::Right => {(first.0+1,first.1)}
            Direction::None=>panic!("方向错误")
        }
    }
    pub fn r#move(&mut self){
        self.old=self.body.pop_front();
        self.body.push_back(self.next_pos())
    }
    pub fn eat(&mut self){
        self.body.push_back(self.next_pos())
    }
    pub fn change_direction(&mut self,d:Direction){
        if d==Direction::None||self.direction.get_opposite()==d {
            return;
        }
        self.direction=d;
    }
    pub fn draw(&self,render:&Render){
        if let Some (left)=&self.old{
            render.draw(left,&' ')
        }
        if let Some(new)=self.body.back(){
            render.draw(new,&'@')
        }
       // for p in self.body.iter(){
       //     render.draw(p,&'@');
       // }
    }
}