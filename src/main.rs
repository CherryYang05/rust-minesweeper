use map::Matrix;

mod event;
mod map;
mod status;

use map::BACKGROUND_COLOR;
use map::TILE_WIDTH;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub static mut DEBUG: bool = true;
fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let window = video
        .window("minesweeper", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();
    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    // 设置背景颜色
    canvas.set_draw_color(BACKGROUND_COLOR);

    // 填充背景颜色
    canvas.clear();

    let mut map = Matrix::new(600 / TILE_WIDTH as usize, 800 / TILE_WIDTH as usize);

    map.draw_map(&mut canvas).unwrap();
    map.generate_mine(&mut canvas, 15);
    map.generate_num(&mut canvas);

    // 更新屏幕
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    // let mut event = Events::new(event_pump);

    'running: loop {
        // event.pump();
        // if let MyEvent::Exit = event.event {
        //     break 'running
        // }

        for event in event_pump.poll_iter() {
            match event {
                // 按下窗口右上角关闭键结束整个程序
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                // 按下键盘上方向键设置 Debug 模式
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => unsafe {
                    DEBUG = !DEBUG;
                    if DEBUG == true {
                        map.draw_map(&mut canvas).unwrap();
                    } else {
                        map.draw_tiles(&mut canvas);
                    }
                    canvas.present();
                },
                _ => {}
            }
        }
    }
}
