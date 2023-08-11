use event::{Events, MyEvent};
use sdl2::{pixels::Color, rect::{Point, Rect}};
mod event;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let window = video
        .window("minesweeper", 1200, 900)
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
    canvas.set_draw_color(Color::RGB(0, 0, 0));

    // 填充背景颜色
    canvas.clear();

    // 设置画笔颜色
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.draw_rect(Rect::new(20, 20, 30, 40)).unwrap();
    canvas.draw_line(Point::new(200, 0), Point::new(200, 400)).unwrap();

    // 更新屏幕
    canvas.present();
    
    
    let event_pump = sdl_context.event_pump().unwrap();
    let mut event = Events::new(event_pump);
    
    'running: loop {
        event.pump();
        if let MyEvent::Exit = event.event {
            break 'running
        }
        // canvas.clear();
        // canvas.present();
    }
}
