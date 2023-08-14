/// 处理地图中相关事件
/// 包含地图结构体，对地图中每个块进行染色填充
/// 实现生成地雷，插旗，显示安全区域等算法
///
use std::path::Path;

use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::surface::Surface;
use sdl2::video::Window;

pub static TILE_WIDTH: u32 = 40; // 每个小格的宽度
pub static BACKGROUND_COLOR: Color = Color::RGB(198, 198, 198);
pub static BOARD_COLOR: Color = Color::RGB(128, 128, 128);

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

// /// 为自定义结构体 Tile 实现 PartialEq trait 以实现结构体之间比较大小
// impl PartialEq for Tile {
//     fn eq(&self, other: &Self) -> bool {

//     }

//     fn ne(&self, other: &Self) -> bool {

//     }
// }

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
                draw_one_tile(canvas, mx, my)?;
            }
        }
        Ok(())
    }

    /// 随机生成地雷
    pub fn generate_mine(&mut self, canvas: &mut Canvas<Window>, mut mine_num: usize) {
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
    pub fn generate_num(&mut self, canvas: &mut Canvas<Window>) {
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

    pub fn draw_tiles(&self, canvas: &mut Canvas<Window>) {
        for mx in 0..self.n {
            for my in 0..self.m {
                let t = &self.data[mx][my];
                set_tile_from_img(canvas, &t, mx, my);
            }
        }
    }
}

/// 渲染单独某一块 Tile 的边框和底色
fn draw_one_tile(canvas: &mut Canvas<Window>, mx: usize, my: usize) -> Result<(), String> {
    canvas.set_draw_color(BACKGROUND_COLOR);
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
            let surface = Surface::load_bmp(Path::new("./assets/mine.bmp")).unwrap();
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
