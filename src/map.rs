use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

/// 处理地图中相关事件
/// 包含地图结构体，对地图中每个块进行染色填充
/// 实现生成地雷，插旗，显示安全区域等算法

pub struct Matrix {
    height: usize,
    width: usize,
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
    UNKNOWN = -5
}

pub static TILE_WIDTH: u32 = 40;      // 每个小格的宽度

impl Matrix {
    pub fn new(height: usize, width: usize) -> Self {
        let data = vec![vec![Tile::UNKNOWN; height]; width];
        Matrix {
            height,
            width,
            data,
        }
    }

    pub fn draw_map(&self, canvas: &mut Canvas<Window>) -> Result<(), String>{
        // let mut map = Map::new(height, width);
        
        for (mx, lines) in self.data.iter().enumerate() {
            for (my, _) in lines.iter().enumerate() {
                // println!("mx = {}, my = {}", mx, my);
                draw_one_tile(canvas, mx, my)?;
            }
        }

        Ok(())
    }
}

fn draw_one_tile(canvas: &mut Canvas<Window>, mx: usize, my: usize) -> Result<(), String>{
    canvas.draw_rect(Rect::new((TILE_WIDTH * my as u32) as i32, (TILE_WIDTH * mx as u32) as i32, TILE_WIDTH, TILE_WIDTH))?;
    Ok(())
}
