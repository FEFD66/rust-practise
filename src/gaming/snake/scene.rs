use std::{u32, thread};
use crate::gaming::snake::snake::Snake;
use crate::gaming::render::{Render, GeneralRender, Point};
use crate::gaming::snake::scene::CheckResult::{Crash, Eat};

use rand::prelude::*;
use std::fs::read_to_string;

pub enum CheckResult {
    None,
    Crash,
    Eat,
}

pub struct Scene{
    height: u32,
    width: u32,
    seeds: Point,
    seed_update: bool
}

impl Scene{
    pub fn check(&mut self,snake:&Snake) -> CheckResult {
        let new = snake.next_pos();
        if self.seed_consume(&new) {
            self.seeds = self.generate_seeds(Some(snake));
            self.seed_update = true;
            Eat
        } else if new.0 == 0 || new.0 == self.width || new.1 == 0 || new.1 == self.height || snake.self_cut() {
            Crash
        } else {
            CheckResult::None
        }
    }
    fn seed_consume(&mut self, place: &Point) -> bool {
        &self.seeds == place
    }
    fn generate_seeds(&self,snake:Option<&Snake>) -> Point {
        let mut rng = thread_rng();
        let mut new;
        loop {
            let x = rng.gen_range(1..self.width);
            let y = rng.gen_range(1..self.height);
            new = (x, y);
            if let Some(s) = snake {
                if !s.contains(&new) {
                    return new;
                }
            } else {
                return new;
            }
        }
    }
}

impl Scene {
    pub fn new() -> Self {
        Scene::with_size(10, 20)
    }
    pub fn with_size(height: u32, width: u32) -> Self {
        let mut s = Scene { height: height - 1, width: width - 1, seeds: (0, 0), seed_update: true, };
        s.seeds = s.generate_seeds(None);
        s
    }
    pub fn with_fullsize() -> Self {
        let render = Render::new();
        let (width, height) = render.get_termsize();
        Scene::with_size(height, width)
    }

    pub fn draw_wall(&self, render: &Render) {
        let n = (self.width + 1) as usize;
        let str = "#".repeat(n);
        render.draw(&(0, 0), &str);
        let ss = format!("{}{}{}", "#", " ".repeat(n - 2), "#");
        for y in 1..self.height {
            render.draw(&(0, y), &ss);
            // thread::sleep(Duration::from_millis(200));
        }
        render.draw(&(0, self.height), &str);
    }

    pub fn draw(&mut self, render: &Render) {
        if !self.seed_update { return; }
        render.draw(&self.seeds, &'*');
        self.seed_update = false;
    }
}