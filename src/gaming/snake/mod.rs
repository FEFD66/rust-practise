
use crate::gaming::snake::render::{Render, GeneralRender};
use std::thread::sleep;
use std::time::Duration;

mod render;

pub fn run(){
    let r=Render::new();
    r.clear();
    for x in 0..20{
        for y in 0..20{
            r.draw((x,y),'@');
            sleep(Duration::from_millis(2000));
        }
    }
}
#[cfg(test)]
mod tests{
    use crate::gaming::snake::render::{Render, GeneralRender};
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