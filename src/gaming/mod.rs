pub mod snake;
mod render;

#[cfg(test)]
mod tests{

   #[test]
   fn game_snake() {
      super::snake::run();
   }
}