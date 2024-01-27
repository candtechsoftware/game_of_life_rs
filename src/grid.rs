use rand::Rng;
use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

use crate::{cell::Cell, universe::Universe};

pub const BACKGROUND_COLOR: Color = Color::RGBA(22, 22, 22, 255);

pub struct Grid {
    pub height: i32,
    pub width: i32,
    pub cell_size: i32,
    pub line_color: Color,
    pub cursor: Rect,
    pub ghost_cursor: Rect,
    pub ghost_cursor_color: Color,
    pub cursor_color: Color,
    pub window_height: i32,
    pub window_width: i32,
    pub universe: Universe,
}

pub fn get_window_size_based_on_cell_size(width: i32, height: i32, cell_size: i32) -> (u32, u32) {
    (
        ((height * cell_size) + 1) as u32,
        ((width * cell_size) + 1) as u32,
    )
}

impl Grid {
    pub fn new() -> Self {
        let height: i32 = 100;
        let width: i32 = 100;
        let cell_size: i32 = 10;
        let (window_height, window_width) =
            get_window_size_based_on_cell_size(width, height, cell_size);
        Self {
            height,
            width,
            window_width: window_width as i32,
            window_height: window_height as i32,
            cell_size,
            cursor_color: Color::RGBA(255, 255, 255, 255),
            ghost_cursor_color: Color::RGBA(44, 44, 44, 255),
            line_color: Color::RGBA(44, 44, 44, 255),
            cursor: Rect::new(
                (width - 1) / 2 * cell_size as i32,
                (height - 1) / 2 * cell_size as i32,
                cell_size as u32,
                cell_size as u32,
            ),
            ghost_cursor: Rect::new(
                (width - 1) / 2 * cell_size,
                (height - 1) / 2 * cell_size,
                cell_size as u32,
                cell_size as u32,
            ),
            universe: Universe::new(width as u32, height as u32),
        }
    }

    pub fn update_cursor(&mut self, x: i32, y: i32) {
        self.cursor.x = (x / self.cell_size) * self.cell_size;
        self.cursor.y = (y / self.cell_size) * self.cell_size;
    }

    pub fn draw_ghost_cursor(&mut self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.ghost_cursor_color);
        canvas.fill_rect(self.ghost_cursor.clone()).unwrap();
    }

    pub fn draw_cell(&mut self, x: i32, y: i32, canvas: &mut Canvas<Window>) {
        let mut rng = rand::thread_rng();
        let r: u8 = rng.gen_range(0..255);
        let g: u8 = rng.gen_range(0..255);
        let b: u8 = rng.gen_range(0..255);
        let color = Color::RGB(r, g, b);
        canvas.set_draw_color(color);
        let cell_rect = Rect::new(
            x * self.cell_size,
            y * self.cell_size,
            self.cell_size as u32,
            self.cell_size as u32,
        );
        canvas.fill_rect(cell_rect).unwrap();
    }

    pub fn draw_cursor(&mut self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.cursor_color);
        canvas.fill_rect(self.cursor.clone()).unwrap()
    }
    pub fn draw_lines(&mut self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.line_color);

        let mut x = 0;
        while x < (1 + self.width * self.cell_size) {
            canvas
                .draw_line(Point::new(x, 0), Point::new(x, self.window_height))
                .unwrap();
            x += self.cell_size;
        }

        let mut y = 0;
        while y < (1 + self.height * self.cell_size) {
            canvas
                .draw_line(Point::new(0, y), Point::new(self.window_width, y))
                .unwrap();
            y += self.cell_size;
        }

        for x in 0..self.width {
            for y in 0..self.height {
                let index = x * self.width + y;
                let cell = self.universe.cells[index as usize].clone();
                match cell {
                    Cell::Alive => self.draw_cell(x, y, canvas),
                    _ => {}
                }
            }
        }
        self.universe.tick();
    }
}
