use sdl2::{event::Event, keyboard::{Keycode, Mod, Scancode}, pixels::Color};
fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let window = video
        .window("minesweeper", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.clear();
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                // 按下窗口右上角关闭键结束整个程序
                Event::Quit { .. } => {
                    break 'running;
                }
                // // 按下键盘上方向键结束整个程序
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }
    }
}
