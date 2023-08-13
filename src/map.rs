/// 处理地图中相关事件
/// 包含地图结构体，对地图中每个块进行染色填充
/// 实现生成地雷，插旗，显示安全区域等算法
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
    MINE = -1,
    FLAG = -2,
    NUM = -3,
    EMPTY = -4,
    UNKNOWN = -5,
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
        // let mut map = Map::new(height, width);

        for (mx, lines) in self.data.iter().enumerate() {
            for (my, _) in lines.iter().enumerate() {
                // println!("mx = {}, my = {}", mx, my);
                // 设置画笔颜色
                canvas.set_draw_color(BOARD_COLOR);
                draw_one_tile(canvas, mx, my)?;
            }
        }

        Ok(())
    }

    /// 随机生成地雷
    pub fn generate_mine(&self, canvas: &mut Canvas<Window>, mut mine_num: usize) {
        let mut rng = rand::thread_rng();
        while mine_num > 0 {
            let r_mx = rng.gen_range(0..self.n);
            let r_my = rng.gen_range(0..self.m);
            if let Tile::MINE = self.data[r_mx][r_my] {
                continue;
            }
            mine_num -= 1;
            self.set_tile_from_img(canvas, Tile::MINE, r_mx, r_my);
        }
    }

    /// 根据 Tile 类型渲染 Tile
    pub fn set_tile_from_img(&self, canvas: &mut Canvas<Window>, t: Tile, mx: usize, my: usize) {
        let texture_creator = canvas.texture_creator();
        match t {
            Tile::MINE => {
                let _surface = Surface::new(40, 40, sdl2::pixels::PixelFormatEnum::RGB24);
                let surface = Surface::load_bmp(Path::new("./assets/mine.bmp")).unwrap();
                let texture = texture_creator
                    .create_texture_from_surface(surface)
                    .unwrap();
                canvas.copy(&texture, None, get_tile_rect(mx, my)).unwrap();
                draw_one_tile(canvas, mx, my).unwrap();
            }
            _ => (),
        }
    }
}

/// 渲染单独某一块 Tile
fn draw_one_tile(canvas: &mut Canvas<Window>, mx: usize, my: usize) -> Result<(), String> {
    canvas.draw_rect(get_tile_rect(mx, my))?;
    Ok(())
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
