mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;
// #[wasm_bindgen]
// // 表示这个函数使用C语言的调用约定，这是WASM中JavaScript交互的标准方式
// extern "C" {
//     fn alert(s: &str);
// }


macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into());
    };
}

// 属性宏，告诉 wasm_bindgen 库，这个函数需要被刀出到JavaScript环境中
#[wasm_bindgen]
// NOTE: repr is short of representation 表示枚举中的每个变体被编码为一个8位的无符号整数
// repr 指定的是 枚举类型的底层表示方式
#[repr(u8)] // QUE: 语法作用。将每个细胞表示为一个字节
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}
// 细胞状态是 0，1 可以比较方便通过假发计算细胞的活邻居

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        }
    }
}
#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Self {
        utils::set_panic_hook();
        let width = 64;
        let height = 64;

        // QUE: 为什么使用 ..width 与 height 相乘
        // NOTE: 默认是 0..6 不包函 6
        // NOTE: 这里的实际意义应该是创建一个 0..width * height 的一个 range
        // NOTE: 这里不知道为什么不能舍弃 0, 否则会编译出错
        let cells = (0..(width * height))
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
                // NOTE: collect 作用是将 iterator collect 成一个Vector
            }).collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    // QUE: 返回类型为什么是一个 const ? 这个为什么能够返回一个指针？
    // NOTE: as_ptr 作用是什么？
    pub fn cells(&self) -> *const Cell {
        // NOTE: as_ptr 返回的是一个指针，指向的是 cells 这个数组的起始地址
        self.cells.as_ptr()
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        // index 为5 的行，实际上是第六行，计算 row * (6 - 1) = row * 5
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    // NOTE: pub 表示这是暴露给JavaScript的
    pub fn tick(&mut self) {
        // QUE: 直接clone是不是太多了, 为什么不直接修改呢？已经传递过来 mut 的引用了
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                log!("cell[{}, {}], is initially {:?} and has {} live neighbors",
                    row, col, cell, live_neighbors
                );

                // NOTE: 自己造一个元组，然后进行匹配
                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                log!("     it becomes {:?}", next_cell);

                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..height * self.width).map(|_i| Cell::Dead).collect();
    }
}


impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // QUE: chunks 是什么方法?
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        // QUE 本来就没有返回值，为什么需要一个 Result ？
        Ok(())
    }
}


impl Universe {
    // QUE: 返回值类型是什么？
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    // 设置指定的细胞为活细胞
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        // cells.cloned().iter() 是否可以？
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }
}
