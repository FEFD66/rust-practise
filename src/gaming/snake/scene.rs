use std::{u32, thread};
use super::snake;
use crate::gaming::snake::snake::Snake;
use crate::gaming::render::{Render, GeneralRender, Point};
use std::time::Duration;

pub struct Scene {
    height: u32,
    width: u32,
    seed:Vec<Point>,
}

impl Scene {
    pub fn step(&self){

    }
    pub fn check(&self, snake: &mut Snake) -> bool {
        let new = snake.next_pos();
        new.0 == 0 && new.0 == self.width && new.1 == 0 && new.1 == self.height
    }
    pub fn draw_wall(&self, render: &Render) {
        let n = self.width as usize;
        let str = "#".repeat(n);
        render.draw(&(0, 0), &str);
        let ss = format!("{}{}{}", "#", " ".repeat(n - 2), "#");
        for y in 1..self.height-1 {
            render.draw(&(0, y), &ss);
            // thread::sleep(Duration::from_millis(200));
        }
        render.draw(&(0, self.height-1), &str);
    }
}

impl Scene {
    pub fn new() -> Self {
        Scene { height: 10, width: 80 }
    }
    pub fn with_size(height: u32, width: u32) -> Self {
        Scene { height, width }
    }
    pub fn with_fullsize() -> Self {
        let render = Render::new();
        let (width, height) = render.get_termsize();
        Scene { height, width: width }
    }
}