use crate::gaming::snake::snake::{Direction, Snake};
use std::sync::mpsc::{Sender, Receiver};
use crate::gaming::render::{Render, GeneralRender};
use crate::gaming::snake::scene::Scene;
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
    Pause,
    Resume,
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
            scene: Scene::with_fullsize(),
            snake: Snake::new(),
            render: Render::new(),
            is_pause: true,
            is_finish: false,
        }
    }
    pub fn init(&self) {
        self.scene.draw_wall(&self.render);
        // self.snake.draw(&self.render);
    }
    pub fn run(&mut self) {
        self.is_pause = false;
        let (tx, rx) = mpsc::channel();
        //启动 输入线程
        thread::spawn(move || handle_input(tx));

        while !self.is_finish {
            //第二层循环用来判断暂停
            while !self.is_pause {
                //输入处理
                self.input(&rx);
                //移动处理
                let crash = self.scene.check(&mut self.snake);
                if crash {
                    self.is_finish = true;
                    break;
                }
                self.snake.r#move();//
                self.snake.draw(&self.render);
                thread::sleep(Duration::from_millis(200));
            }
            thread::yield_now();
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
                        Command::Resume => {
                            self.is_pause = false;
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
        let mut need_send = true;
        if let Ok(ch) = x.read_char() {
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
            need_send = true;
        }
        thread::sleep(Duration::from_millis(100));
    }
}