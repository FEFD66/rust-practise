use crate::gaming::snake::snake::{Direction, Snake};
use std::sync::mpsc::{Sender, Receiver};
use crate::gaming::render::{Render, GeneralRender};
use crate::gaming::snake::scene::{Scene, CheckResult};
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

enum Signal {
    None,
    Dir(Direction),
    Cmd(Command),
}

enum Command {
    Quit,
    Pause
}

pub struct App {
    scene: Scene,
    snake: Snake,
    render: Render,
    is_pause: bool,
    is_finish: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            // scene: Scene::with_size(10,10),
            scene:Scene::with_fullsize(),
            snake: Snake::new(),
            render: Render::new(),
            is_pause: true,
            is_finish: false,
        }
    }
    pub fn init(& mut self) {
        self.scene.draw_wall(&self.render);
        // self.snake.draw(&self.render);
    }
    pub fn run(&mut self) {
        self.is_pause = false;
        let (tx, rx) = mpsc::channel();
        //启动 输入线程
        thread::spawn(move || handle_input(tx));

        while !self.is_finish {
            //输入处理
            self.input(&rx);
            //场景处理
            self.scene.draw(&self.render);
            //移动处理
            let crash = self.scene.check(&self.snake);
            match crash{
                CheckResult::None => {self.snake.r#move()}
                CheckResult::Crash => {self.is_finish=true;break;}
                CheckResult::Eat => {self.snake.eat()}
            }
            self.snake.draw(&self.render);
            thread::sleep(Duration::from_millis(200));
        }
    }
    fn input(&mut self, recv: &Receiver<Signal>) {
        let mut direction = Direction::None;
        //循环以清楚管道内冗余的信息获得最新信息
        loop {
            if let Ok(sig) = recv.try_recv() {
                // eprintln!("改变方向");
                match sig {
                    Signal::Dir(dir) => {
                        direction = dir;
                    }
                    Signal::Cmd(cmd) => match cmd {
                        Command::Quit => { return; }
                        Command::Pause => {
                            self.is_pause = !self.is_pause;
                            break;
                        }
                    }
                    Signal::None => {}
                }
            } else {
                break;
            }
        }
        self.snake.change_direction(direction);
    }
}

fn handle_input(sender: Sender<Signal>) {
    let x = Render::new();
    loop {
        let mut signal = Signal::None;
        let mut need_send ;
        if let Ok(ch) = x.read_char() {
            need_send = true;
            match ch {
                'w' => signal = Signal::Dir(Direction::Up),
                'a' => signal = Signal::Dir(Direction::Left),
                'd' => signal = Signal::Dir(Direction::Right),
                's' => signal = Signal::Dir(Direction::Down),
                'q' => signal = Signal::Cmd(Command::Quit),
                'p' => signal = Signal::Cmd(Command::Pause),
                _ => { need_send = false }
            }
            if need_send {
                sender.send(signal).unwrap();
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
}