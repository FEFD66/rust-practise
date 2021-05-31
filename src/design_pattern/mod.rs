//设计模式
mod singleton;


#[cfg(test)]
mod test{
    use crate::design_pattern::singleton::SingleConfig;
    use std::thread::spawn;
    use std::thread;

    #[test]
    fn sigleton(){
        println!("单例模式");
        let sc=SingleConfig::new_sigletone();
        println!("{:?}",sc);
        let sc=SingleConfig::new().as_mut().unwrap();
        println!("{:?}",sc);
        {
            let mut sc=SingleConfig::new().as_mut().unwrap();
            sc.file_config="new config".to_string();
        }
        println!("{:?}",sc);

        let handle=thread::spawn(move ||{
            sc.file_config="new thread config".to_string();
        });

        handle.join().unwrap();
        println!("{:?}",SingleConfig::new().as_mut().unwrap());

    }
}