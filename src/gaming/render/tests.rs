use super::{Render, GeneralRender};
use std::thread::sleep;
use std::time::Duration;

#[test]
fn render() {
    let r = Render::new();
    r.clear();
    let (width, height) = r.get_termsize();

    for x in 0..width {
        for y in 0..height {
            r.draw(&(x, y), &'@');
            sleep(Duration::from_nanos(1));
        }
    }
}

#[test]
fn input() {
    let r = Render::new();
    let mut x = ' ';
    while x != 'q' {
        print!("newline:");
        x = r.read_char().unwrap_or('@');
        println!("{}", x);
        sleep(Duration::from_millis(100));
    }
}