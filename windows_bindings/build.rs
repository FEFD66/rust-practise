// https://microsoft.github.io/windows-docs-rs/doc/bindings
fn main(){
    windows::build!{
        Windows::Win32::System::Console::*,
        Windows::Win32::System::SystemServices::CHAR,
        Windows::Win32::Foundation::*,
    };
}