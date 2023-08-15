/// 处理地图中相关事件
/// 包含地图结构体，对地图中每个块进行染色填充
/// 实现生成地雷，插旗，显示安全区域等算法
///
use std::path::Path;

use rand::Rng;
use sdl2::mouse::MouseState;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::surface::{Surface, SurfaceRef};
use sdl2::sys::{SDL_GetMouseState, SDL_MapRGB, SDL_SetColorKey};
use sdl2::video::Window;

pub static TILE_WIDTH: u32 = 40; // 每个小格的宽度
pub static BACKGROUND_COLOR: Color = Color::RGB(198, 198, 198);
pub static BOARD_COLOR: Color = Color::RGB(128, 128, 128);
pub static SAFE_COLOR: Color = Color::RGB(165, 165, 165);
pub static HOVER_COLOR: Color = Color::RGB(220, 220, 220);

pub struct Matrix {
    n: usize,
    m: usize, // n 行 m 列的矩阵
    data: Vec<Vec<Tile>>,
}

type Map = Matrix;

// 地图中每个小格
#[derive(Clone)]
pub enum Tile {
    UNKNOWN, // 初始化之后的状态
    MINE,    // 地雷
    FLAG,    // 鼠标右键插旗子
    NUM(u8), // 地雷数量标记
    SAFE,    // 没有数字的 Tile
}

/// 为自定义结构体 Tile 实现 PartialEq trait 以实现结构体之间比较大小
impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::FLAG, Self::FLAG) => true,
            (Self::MINE, Self::MINE) => true,
            (Self::UNKNOWN, Self::UNKNOWN) => true,
            (Self::NUM(_), Self::NUM(_)) => true,
            (Self::SAFE, Self::SAFE) => true,
            _ => false,
        }
    }

    // fn ne(&self, other: &Self) -> bool {

    // }
}

impl Matrix {
    // n * m 矩阵
    pub fn new(n: usize, m: usize) -> Self {
        let data = vec![vec![Tile::UNKNOWN; m]; n];
        Matrix { n, m, data }
    }

    /// 绘制整个地图
    pub fn draw_map(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        for mx in 0..self.n {
            for my in 0..self.m {
                draw_one_tile(canvas, BACKGROUND_COLOR, mx, my)?;
            }
        }
        Ok(())
    }

    /// 随机生成地雷
    pub fn generate_mine(&mut self, mut mine_num: usize) {
        let mut rng = rand::thread_rng();
        while mine_num > 0 {
            let r_mx = rng.gen_range(0..self.n);
            let r_my = rng.gen_range(0..self.m);
            if let Tile::MINE = self.data[r_mx][r_my] {
                continue;
            }
            mine_num -= 1;
            self.data[r_mx][r_my] = Tile::MINE;
            // set_tile_from_img(canvas, &Tile::MINE, r_mx, r_my);
        }
    }

    /// 生成地雷周围的数字
    pub fn generate_num(&mut self) {
        // let mut mine_num: Vec<Vec<u8>> = vec![vec![0; self.m]; self.n];
        for mx in 0..self.n {
            for my in 0..self.m {
                let mut mine_num: u8 = 0;
                if let Tile::UNKNOWN = self.data[mx][my] {
                    for x in -1..=1 {
                        for y in -1..=1 {
                            let nx = mx as isize + x;
                            let ny = my as isize + y;
                            if nx >= 0 && nx < self.n as isize && ny >= 0 && ny < self.m as isize {
                                if let Tile::MINE = self.data[nx as usize][ny as usize] {
                                    mine_num += 1;
                                }
                            }
                        }
                    }
                    if mine_num == 0 {
                        self.data[mx][my] = Tile::SAFE;
                    } else {
                        self.data[mx][my] = Tile::NUM(mine_num);
                    }
                }
                // set_tile_from_img(canvas, &Tile::NUM(mine_num), mx, my);
            }
        }
    }

    // 给所有 Tile 设置 img 或背景
    pub fn draw_tiles(&self, canvas: &mut Canvas<Window>) {
        for mx in 0..self.n {
            for my in 0..self.m {
                let t = &self.data[mx][my];
                set_tile_from_img(canvas, t, mx, my);
            }
        }
    }

    // 插小旗子，再次点击取消插旗
    pub fn set_flag(&mut self, canvas: &mut Canvas<Window>, mouse_state: &MouseState) {
        let tile = mouse_key_in_which_tile(mouse_state.x(), mouse_state.y());
        // let tile_state = self.data[tile.0][tile.1];
        // 如果当前 Tile 不是旗子，则插上旗子，否则取消旗子
        if self.data[tile.0][tile.1] != Tile::FLAG {
            self.data[tile.0][tile.1] = Tile::FLAG;
            set_tile_from_img(canvas, &Tile::FLAG, tile.0, tile.1);
        } else {
            self.data[tile.0][tile.1] = Tile::SAFE;
            set_tile_from_img(canvas, &Tile::SAFE, tile.0, tile.1)
        }
    }

    // 左键点击显示 Tile，并显示连续的安全区域
    pub fn show_tile(&mut self, canvas: &mut Canvas<Window>, mouse_state: &MouseState) {
        let tile = mouse_key_in_which_tile(mouse_state.x(), mouse_state.y());
        set_tile_from_img(canvas, &self.data[tile.0][tile.1], tile.0, tile.1);
    }
}

/// 渲染单独某一块 Tile 的边框和底色
fn draw_one_tile(canvas: &mut Canvas<Window>, color: Color, mx: usize, my: usize) -> Result<(), String> {
    // let tile: (u32, u32);
    // let mouse_state = unsafe { SDL_GetMouseState(tile.0, tile.1) };
    // let tile = mouse_key_in_which_tile(mouse_state.x(), mouse_state.y());
    // if let (mx, my) = tile {
    //     canvas.set_draw_color(HOVER_COLOR);
    // } else {
    // }
    canvas.set_draw_color(color);
    // canvas.fill_rect(get_tile_rect(tile.0, tile.1)).unwrap();
    canvas.fill_rect(get_tile_rect(mx, my))?;
    canvas.set_draw_color(BOARD_COLOR);
    canvas.draw_rect(get_tile_rect(mx, my))?;
    Ok(())
}

/// 根据 Tile 类型渲染 Tile
fn set_tile_from_img(canvas: &mut Canvas<Window>, t: &Tile, mx: usize, my: usize) {
    let texture_creator = canvas.texture_creator();
    match t {
        Tile::MINE => {
            // SDL_SetColorKey(surface, flag, key)
            let mut surface = Surface::load_bmp(Path::new("./assets/mine.bmp")).unwrap();
            // SurfaceRef::set_color_key(&mut surface, true, BACKGROUND_COLOR).unwrap();
            // SDL_MapRGB(format, r, g, b)
            // let texture = texture_creator.
            let texture = texture_creator
                .create_texture_from_surface(surface)
                .unwrap();
            canvas.copy(&texture, None, get_tile_rect(mx, my)).unwrap();
        }
        Tile::NUM(num) => {
            if num > &0 {
                let surface =
                    Surface::load_bmp(Path::new(format!("./assets/{}.bmp", num).as_str())).unwrap();
                let texture = texture_creator
                    .create_texture_from_surface(surface)
                    .unwrap();
                canvas.copy(&texture, None, get_tile_rect(mx, my)).unwrap();
            }
        }
        Tile::FLAG => {
            let surface = Surface::load_bmp(Path::new("./assets/flag.bmp")).unwrap();
            let texture = texture_creator
                .create_texture_from_surface(surface)
                .unwrap();
            canvas.copy(&texture, None, get_tile_rect(mx, my)).unwrap();
        }
        Tile::SAFE => {
            draw_one_tile(canvas, SAFE_COLOR, mx, my).unwrap();
        }
        _ => (),
    }

    // 重新描一下边框
    canvas.set_draw_color(BOARD_COLOR);
    canvas.draw_rect(get_tile_rect(mx, my)).unwrap();
}

/// 获取对应 Tile 所在的 Rect
fn get_tile_rect(x: usize, y: usize) -> Rect {
    Rect::new(
        (TILE_WIDTH * y as u32) as i32,
        (TILE_WIDTH * x as u32) as i32,
        TILE_WIDTH,
        TILE_WIDTH,
    )
}

pub fn set_tile_highlight(canvas: &mut Canvas<Window>, mouse_state: MouseState) {
    let tile = mouse_key_in_which_tile(mouse_state.x(), mouse_state.y());
    canvas.set_draw_color(HOVER_COLOR);
    canvas.fill_rect(get_tile_rect(tile.0, tile.1)).unwrap();
    canvas.set_draw_color(BOARD_COLOR);
    canvas.draw_rect(get_tile_rect(tile.0, tile.1)).unwrap();
}

/// 检测当前鼠标位于哪一块 Tile 上
fn mouse_key_in_which_tile(x: i32, y: i32) -> (usize, usize) {
    (
        y as usize / TILE_WIDTH as usize,
        x as usize / TILE_WIDTH as usize,
    )
}
