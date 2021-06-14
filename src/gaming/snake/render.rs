
#[cfg(target_os = "windows")]
use windows_bindings::Windows::Win32::{System::Console::*,Foundation};
use std::alloc::handle_alloc_error;
use std::fmt;
use std::fmt::Display;

pub type Point=(u32,u32);
pub trait GeneralRender{
    fn draw<T:Display>(&self,p:Point,c:T);
    fn clear(&self);
}
pub struct Render {
    handle: Foundation::HANDLE
}
impl Render{
    pub fn new()->Self{
        unsafe {
            let handle=GetStdHandle(STD_OUTPUT_HANDLE);
            Render{handle}
        }
    }
}
#[cfg(target_os = "windows")]
impl GeneralRender for Render{
    fn draw<T: Display>(&self, p: (u32, u32), c: T) {
            unsafe {
                // let handle=GetStdHandle(STD_OUTPUT_HANDLE);
                let coord= COORD{X:p.0 as i16,Y:p.1 as i16};
                let bool:Foundation::BOOL=SetConsoleCursorPosition(self.handle,coord);

            }
            print!("{}",c);
    }

    fn clear(&self) {
        static BLANK_LINE:&'static str="                                                                                                                            ";
        for y in 0..20 {
            self.draw((0,y),BLANK_LINE);
        }
    }
}
#[cfg(target_os = "linux")]
impl GeneralRender for Render{
    fn draw(p: (u32, u32), c: char) {
        todo!()
    }
}
