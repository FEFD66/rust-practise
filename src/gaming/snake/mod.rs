use app::App;

mod scene;
mod snake;
mod app;

pub fn run() {
    let mut app =App::new();
    app.init();
    app.run();
}

// #[cfg(test)]
// mod tests{
//     use crate::gaming::render::{Render, GeneralRender};
//     use crate::gaming::snake::snake::Snake;
//     use crate::gaming::snake::scene::Scene;
//
//     #[test]
//     fn draw_snake(){
//         let r=Render::new();
//         let s=Snake::new();
//         let scene=Scene::new();
//         scene.draw_wall(&r);
//         s.draw(&r);
//     }
// }
