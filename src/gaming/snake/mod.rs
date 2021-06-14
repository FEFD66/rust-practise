
use crate::gaming::render::{Render, GeneralRender};
use std::thread::sleep;
use std::time::Duration;

mod scene;
mod snake;

pub fn run(){
    let r=Render::new();
    r.clear();
    for x in 0..15{
        for y in 0..80{
            r.draw((x,y),'@');
            sleep(Duration::from_millis(100));
        }
    }
}
#[cfg(test)]
mod tests{
    use crate::gaming::render::{Render, GeneralRender};
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn render(){
        let r=Render::new();
        r.clear();
        for x in 0..20{
            for y in 0..20{
                r.draw((x,y),'@');
                sleep(Duration::from_millis(2000));
            }
        }
    }
}