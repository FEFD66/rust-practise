pub mod snake;
mod render;

#[cfg(test)]
mod tests{
   use crate::gaming::render::{GeneralRender, Render};
   use std::thread::sleep;
   use std::time::Duration;

   #[test]
   fn game_snake() {
      super::snake::run();
   }
}