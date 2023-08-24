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
use sdl2::surface::Surface;
use sdl2::video::Window;

use crate::MARGIN;

pub static TILE_WIDTH: u32 = 40; // 每个小格的宽度
pub static BACKGROUND_COLOR: Color = Color::RGB(198, 198, 198);
pub static BOARD_COLOR: Color = Color::RGB(128, 128, 128);
pub static SAFE_COLOR: Color = Color::RGB(165, 165, 165);
// pub static HOVER_COLOR: Color = Color::RGB(220, 220, 220);

pub struct Matrix {
    n: usize,
    m: usize, // n 行 m 列的矩阵
    data: Vec<Vec<Tile>>,
    flag: Vec<Vec<bool>>,
    shown: Vec<Vec<bool>>,
    // flag_num: usize,
}

// type Map = Matrix;

// 地图中每个小格
#[derive(Clone)]
pub enum Tile {
    UNKNOWN,  // 初始化之后的状态
    MINE,     // 地雷
    RED_MINE, // 踩到的地雷
    FLAG,     // 鼠标右键插旗子
    NUM(u8),  // 地雷数量标记
    SAFE,     // 没有数字的 Tile
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
        let flag = vec![vec![false; m]; n];
        let shown = vec![vec![false; m]; n];
        Matrix {
            n,
            m,
            data,
            flag,
            shown,
            // flag_num: mine_num,
        }
    }

    // 重置 map
    pub fn renew(&mut self, n: usize, m: usize) {
        let data = vec![Tile::UNKNOWN; m];
        let flag = vec![false; m];
        let shown = vec![false; m];
        self.data.fill(data);
        self.flag.fill(flag);
        self.shown.fill(shown);
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

    /// 给所有 Tile 设置 img 或背景
    pub fn draw_tiles(&mut self, canvas: &mut Canvas<Window>, debug: bool) {
        for mx in 0..self.n {
            for my in 0..self.m {
                let t = &self.data[mx][my];
                if debug {
                    set_tile_from_img(canvas, t, mx, my);
                    self.shown[mx][my] = false;
                } else {
                    if self.shown[mx][my] {
                        set_tile_from_img(canvas, t, mx, my);
                    }
                }
            }
        }
    }

    /// 插小旗子，再次点击取消插旗
    pub fn set_flag(&mut self, canvas: &mut Canvas<Window>, mouse_state: &MouseState) {
        let tile = mouse_key_in_which_tile(mouse_state.x(), mouse_state.y());
        match tile {
            Ok((x, y)) => {
                // let tile_state = self.data[tile.0][tile.1];
                // 如果当前 Tile 不是旗子，则插上旗子，否则取消旗子
                if self.shown[x][y] == false {
                    if self.flag[x][y] == false {
                        self.flag[x][y] = true;
                        set_tile_from_img(canvas, &Tile::FLAG, x, y);
                    } else {
                        self.flag[x][y] = false;
                        draw_one_tile(canvas, BACKGROUND_COLOR, x, y).unwrap();
                    }
                }
            }
            Err(_) => {}
        }
    }

    /// 左键点击显示 Tile，并显示连续的安全区域
    pub fn show_tile(&mut self, canvas: &mut Canvas<Window>, mouse_state: &MouseState) -> bool {
        let tile = mouse_key_in_which_tile(mouse_state.x(), mouse_state.y());
        match tile {
            Ok((x, y)) => {
                if self.shown[x][y] == false {
                    self.shown[x][y] = true;
                    // 踩雷之后游戏结束
                    if self.data[x][y] == Tile::MINE {
                        self.draw_tiles(canvas, true);
                        set_tile_from_img(canvas, &Tile::RED_MINE, x, y);
                        self.data[x][y] = Tile::RED_MINE;
                        return false;
                    } else if self.data[x][y] == Tile::SAFE {
                        self.flood(x, y);
                    }
                    // set_tile_from_img(canvas, &self.data[x][y], x, y);
                }
            }
            Err(_) => {}
        }
        true
    }

    /// DFS 算法检测连续的安全区域，
    /// 有数字的 Tile 看成高度，SAFE 区域看成高度为 0。
    /// 算法可以看成往一个凹下去的地方倒水，最多可以覆盖多少块 Tile。
    fn flood(&mut self, x: usize, y: usize) {
        if self.data[x][y] == Tile::NUM(0) {
            self.shown[x][y] = true;
            return;
        }
        self.shown[x][y] = true;
        for nx in -1..=1 {
            for ny in -1..=1 {
                if i32::abs(nx as i32) != i32::abs(ny as i32) {
                    let cx = x as isize + nx;
                    let cy = y as isize + ny;
                    if cx >= 0 && cx < self.n as isize && cy >= 0 && cy < self.m as isize {
                        if self.shown[cx as usize][cy as usize] == false {
                            self.shown[x][y] = true;
                            self.flood(cx as usize, cy as usize);
                        }
                    }
                }
            }
        }
    }

    /// 检查是否已经排完了所有的雷，若排完，游戏获胜
    pub fn check(&self) -> bool {
        // let is_success = false;
        for i in 0..self.n {
            for j in 0..self.m {
                if self.shown[i][j] == false && self.data[i][j] != Tile::MINE {
                    return false;
                }
            }
        }
        true
    }

    pub fn set_shown(&mut self, flag: bool) {
        for i in 0..self.n {
            for j in 0..self.m {
                self.shown[i][j] = flag;
            }
        }
    }
}

/// 渲染单独某一块 Tile 的边框和底色
fn draw_one_tile(
    canvas: &mut Canvas<Window>,
    color: Color,
    mx: usize,
    my: usize,
) -> Result<(), String> {
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
            let surface = Surface::load_bmp(Path::new("./assets/mine.bmp")).unwrap();
            // SurfaceRef::set_color_key(&mut surface, true, BACKGROUND_COLOR).unwrap();
            // SDL_MapRGB(format, r, g, b)
            // let texture = texture_creator.
            let texture = texture_creator
                .create_texture_from_surface(surface)
                .unwrap();
            canvas.copy(&texture, None, get_tile_rect(mx, my)).unwrap();
        }

        Tile::RED_MINE => {
            let surface = Surface::load_bmp(Path::new("./assets/mine_red.bmp")).unwrap();
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
/// 这里的 x 和 y 表示 x 行 y 列
fn get_tile_rect(x: usize, y: usize) -> Rect {
    Rect::new(
        (TILE_WIDTH * y as u32) as i32,
        (TILE_WIDTH * x as u32 + MARGIN as u32) as i32,
        TILE_WIDTH,
        TILE_WIDTH,
    )
}

/// 检测当前鼠标位于哪一块 Tile 上
/// 鼠标的坐标是按照横轴为 x，纵轴为 y 来计算
fn mouse_key_in_which_tile(x: i32, y: i32) -> Result<(usize, usize), String> {
    if y - MARGIN as i32 >= 0 {
        Ok((
            (y - MARGIN as i32) as usize / TILE_WIDTH as usize,
            x as usize / TILE_WIDTH as usize,
        ))
    } else {
        Err("Err".to_owned())
    }
}
