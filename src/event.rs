use sdl2::{event::Event, keyboard::Keycode, EventPump};

use crate::DEBUG;

// 自定义事件结构体，用来存储可能执行的操作(包括关闭窗口，打开 Debug 模式等)
pub struct Events {
    pump: EventPump,
    pub event: MyEvent,
}

// 自定义事件
pub enum MyEvent {
    None,
    Exit,
    Debug,
}

impl Events {
    pub fn new(pump: EventPump) -> Self {
        Self {
            pump,
            event: MyEvent::None,
        }
    }

    // 监测事件
    pub fn pump(&mut self) {
        for event in self.pump.poll_iter() {
            match event {
                // 按下窗口右上角关闭键结束整个程序
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.event = MyEvent::Exit,
                // 按下键盘上方向键设置 Debug 模式
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    self.event = MyEvent::Debug;
                    unsafe {
                        DEBUG = !DEBUG;
                        // println!("debug = {}", DEBUG);
                    }
                }
                _ => {}
            }
        }
    }
}
