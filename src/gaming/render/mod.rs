#[cfg(test)]
mod tests;

#[cfg(target_os = "windows")]
use std::fmt::Display;

pub type Point = (u32, u32);

// pub type Rect = (Point, Point);

pub trait GeneralRender {
    fn new() -> Self;
    fn draw<T: Display>(&self, p: &Point, c: &T);
    fn clear(&self);
    fn read_char(&self) -> Result<char, ()>;
    fn get_termsize(&self) -> Point;
}

#[cfg(windows)]
pub type Render = windows_render::Render;
#[cfg(target_os = "linux")]
pub type Render = linux_render::Render;

#[cfg(windows)]
mod windows_render {
    use crate::gaming::render::GeneralRender;
    use std::fmt::Display;
    use windows_bindings::Windows::Win32::{System::Console::*, Foundation};

    pub struct Render {
        output: Foundation::HANDLE,
        input: Foundation::HANDLE,
    }

    impl GeneralRender for Render {
        fn new() -> Self {
            unsafe {
                let output = GetStdHandle(STD_OUTPUT_HANDLE);
                if output.is_invalid() {
                    panic!("终端不支持")
                }
                let input = GetStdHandle(STD_INPUT_HANDLE);
                if input.is_invalid() {
                    panic!("终端不支持")
                }
                Render { output, input }
            }
        }
        fn draw<T: Display>(&self, p: &(u32, u32), c: &T) {
            unsafe {
                let coord = COORD { X: p.0 as i16, Y: p.1 as i16 };
                let _: Foundation::BOOL = SetConsoleCursorPosition(self.output, coord);

            }
            eprint!("{}", c);
        }

        fn clear(&self) {
            static BLANK_LINE: &'static str = "                                                                                      ";
            for y in 0..50 {
                self.draw(&(0, y), &BLANK_LINE);
            }
        }

        fn read_char(&self) -> Result<char, ()> {
            let mut record = INPUT_RECORD {
                EventType: 0,
                Event: INPUT_RECORD_0 {
                    KeyEvent: KEY_EVENT_RECORD {
                        bKeyDown: Foundation::BOOL(0),
                        wRepeatCount: 0,
                        wVirtualKeyCode: 0,
                        wVirtualScanCode: 0,
                        uChar: KEY_EVENT_RECORD_0 {
                            UnicodeChar: 0
                        },
                        dwControlKeyState: 0,
                    }
                },
            };
            let mut b: u32 = 0;

            unsafe {
                loop {
                    ReadConsoleInputA(self.input, &mut record, 1, &mut b).as_bool();
                    if b != 0 && KEY_EVENT == record.EventType as u32 {
                        let key_event = record.Event.KeyEvent;
                        if key_event.bKeyDown.as_bool() {
                            return Ok(key_event.uChar.AsciiChar.0 as char);
                        }
                    }
                    // else{
                    //     if(b!=0){
                    //         println!("不是:{}",record.EventType)
                    //     }else {
                    //         println!("空");
                    //     }
                    // }
                }
            }
        }

        fn get_termsize(&self) -> (u32, u32) {
            unsafe {
                let z: i16 = 0;
                let mut info = CONSOLE_SCREEN_BUFFER_INFO {
                    dwSize: COORD { X: z, Y: z },
                    dwCursorPosition: COORD { X: z, Y: z },
                    wAttributes: 0,
                    srWindow: SMALL_RECT { Left: z, Top: z, Right: z, Bottom: z },
                    dwMaximumWindowSize: COORD { X: z, Y: z },
                };
                GetConsoleScreenBufferInfo(self.output, &mut info).unwrap();
                (info.dwSize.X as u32, info.dwSize.Y as u32)
            }
        }
    }
}


#[cfg(target_os = "linux")]
impl GeneralRender for Render {
    fn draw<T: Display>(&self, p: (u32, u32), c: T) {
        eprint!("\x1b[{};{}H{}", p.0, p.1, c);
    }

    fn clear(&self) {
        eprintln!("\x1b[2J");
    }
}
