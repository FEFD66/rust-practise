
#[cfg(target_os = "windows")]
use windows_bindings::Windows::Win32::{System::Console::*,Foundation};

use std::fmt::Display;

pub type Point=(u32,u32);
pub trait GeneralRender{
    fn draw<T:Display>(&self,p:Point,c:T);
    fn clear(&self);
}
pub struct Render {
}
impl Render{
    pub fn new()->Self{
            Render{}
    }
}
#[cfg(target_os = "windows")]
impl GeneralRender for Render{
    fn draw<T: Display>(&self, p: (u32, u32), c: T) {
            unsafe {
                let handle=GetStdHandle(STD_OUTPUT_HANDLE);
                let coord= COORD{X:p.0 as i16,Y:p.1 as i16};
                let bool:Foundation::BOOL=SetConsoleCursorPosition(handle,coord);

            }
            print!("{}",c);
    }

    fn clear(&self) {
        todo!();
        static BLANK_LINE:&'static str="                                                                                                                            ";
        for y in 0..20 {
            self.draw((0,y),BLANK_LINE);
        }
    }
}

#[cfg(target_os = "linux")]
impl GeneralRender for Render{
    fn draw<T:Display>(&self,p: (u32, u32), c: T) {
        eprint!("\x1b[{};{}H{}",p.0,p.1,c);
    }

    fn clear(&self) {
       eprintln!("\x1b[2J");
    }
}
