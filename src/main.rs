mod event;
mod map;
mod status;

use std::path::Path;

use map::Matrix;
use map::BACKGROUND_COLOR;
use map::BOARD_COLOR;
use map::TILE_WIDTH;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::surface::Surface;
use sdl2::video::Window;

pub static mut DEBUG: bool = true;
pub static MARGIN: usize = 60;
static MINE_NUM: usize = 5;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let window = video
        .window("minesweeper", 800, (600 + MARGIN) as u32)
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
    // 画信息栏的边框
    draw_info(&mut canvas);

    let mut map = Matrix::new(
        600 / TILE_WIDTH as usize,
        800 / TILE_WIDTH as usize,
    );

    map.draw_map(&mut canvas).unwrap();
    map.generate_mine(MINE_NUM);
    map.generate_num();
    show_start(&mut canvas);

    // 更新屏幕
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    // let mut event = Events::new(event_pump);

    'running: loop {
        // event.pump();
        // if let MyEvent::Exit = event.event {
        //     break 'running
        // }

        let mouse_state = event_pump.mouse_state();

        for event in event_pump.poll_iter() {
            match event {
                // 按下窗口右上角关闭键结束整个程序
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                // 按键盘上方向键设置 Debug 模式
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => unsafe {
                    DEBUG = !DEBUG;
                    if DEBUG == true {
                        map.draw_map(&mut canvas).unwrap();
                    } else {
                        map.draw_tiles(&mut canvas, true);
                    }
                },

                // 按键盘下方向键重新生成地雷
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    // TODO
                }

                // 鼠标左键按下显示 Tile
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    if mouse_state.x() >= 375
                        && mouse_state.x() <= 425
                        && mouse_state.y() >= 5
                        && mouse_state.y() <= 55
                    {
                        show_start(&mut canvas);
                        // 重开
                        map.renew(600 / TILE_WIDTH as usize, 800 / TILE_WIDTH as usize);
                        map.draw_map(&mut canvas).unwrap();
                        map.generate_mine(MINE_NUM);
                        map.generate_num();
                    } else {
                        let is_goon = map.show_tile(&mut canvas, &mouse_state);
                        map.draw_tiles(&mut canvas, false);
                        // 如果游戏结束，则点击图标重开
                        if !is_goon {
                            show_end(&mut canvas);
                            map.set_shown(true);
                        }
                        if map.check() {
                            show_win(&mut canvas);
                            map.set_shown(true);
                        }
                    }
                }

                // 鼠标右键按下插小旗子
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Right,
                    ..
                } => {
                    map.set_flag(&mut canvas, &mouse_state);
                }

                _ => {}
            }
        }
        canvas.present();
    }
}

fn draw_info(canvas: &mut Canvas<Window>) {
    let rect = Rect::new(0, 0, 800, 60);
    canvas.set_draw_color(BOARD_COLOR);
    canvas.draw_rect(rect).unwrap();
}

fn show_win(canvas: &mut Canvas<Window>) {
    let texture_creator = canvas.texture_creator();
    let surface = Surface::load_bmp(Path::new("./assets/win.bmp")).unwrap();
    let texture = texture_creator
        .create_texture_from_surface(surface)
        .unwrap();
    canvas
        .copy(&texture, None, Rect::new(375, 5, 50, 50))
        .unwrap();
    canvas.set_draw_color(BOARD_COLOR);
    canvas.draw_rect(Rect::new(375, 5, 50, 50)).unwrap();
}

fn show_start(canvas: &mut Canvas<Window>) {
    let texture_creator = canvas.texture_creator();
    let surface = Surface::load_bmp(Path::new("./assets/start.bmp")).unwrap();
    let texture = texture_creator
        .create_texture_from_surface(surface)
        .unwrap();
    canvas
        .copy(&texture, None, Rect::new(375, 5, 50, 50))
        .unwrap();
    canvas.set_draw_color(BOARD_COLOR);
    canvas.draw_rect(Rect::new(375, 5, 50, 50)).unwrap();
}

fn show_end(canvas: &mut Canvas<Window>) {
    let texture_creator = canvas.texture_creator();
    let surface = Surface::load_bmp(Path::new("./assets/game_over.bmp")).unwrap();
    let texture = texture_creator
        .create_texture_from_surface(surface)
        .unwrap();
    canvas
        .copy(&texture, None, Rect::new(375, 5, 50, 50))
        .unwrap();
    canvas.set_draw_color(BOARD_COLOR);
    canvas.draw_rect(Rect::new(375, 5, 50, 50)).unwrap();
}
