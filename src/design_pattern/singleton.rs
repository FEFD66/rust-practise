// 单例模式
//只是保证唯一性
// 只要可以保证唯一性，就不必拘泥于单例模式

//引入标准库
use std::sync::{Mutex,Once};
use std::mem::MaybeUninit;

#[derive(Debug)]
pub struct SingleConfig {
    pub file_config:String
}
impl SingleConfig {
    //单例函数 保证唯一性
    // Maybeunit创建 未被初始化的内存
    pub fn new_sigletone()->&'static Mutex<SingleConfig>{
        static mut CONFIG:MaybeUninit<Mutex<SingleConfig>>=MaybeUninit::uninit();
        static ONCE:Once=Once::new();
        ONCE.call_once(||unsafe{
            CONFIG.as_mut_ptr().write(Mutex::new(SingleConfig{
                file_config:"nihao ".to_string()
            }));
        });
        unsafe {&*CONFIG.as_ptr()}
    }

    pub fn new()->&'static mut Option<Self>{
        static mut CONFIG: Option<SingleConfig>=None;
        unsafe {
            if let None=CONFIG{
                CONFIG=Some(SingleConfig{
                    file_config:"simple singleton".to_string()
                });
            }
            &mut CONFIG
        }
    }
}
