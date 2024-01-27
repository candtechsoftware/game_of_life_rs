extern crate sdl2;

mod cell;
mod grid;
mod universe;

use grid::Grid;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut grid = Grid::new();
    let (width, height) = (grid.window_width, grid.window_height);
    let window = video_subsystem
        .window("Liquid Sim", width as u32, height as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(grid::BACKGROUND_COLOR);
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut mouse_active = false;
    let mut mouse_hover = true;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseButtonDown { x, y, .. } => grid.update_cursor(x, y),
                Event::MouseMotion { x, y, .. } => {
                    grid.update_cursor(x, y);
                    if !mouse_active {
                        mouse_active = true;
                    }
                }
                Event::Window { win_event, .. } => {
                    if win_event == WindowEvent::Enter && !mouse_hover {
                        mouse_hover = true;
                    } else if win_event == WindowEvent::Leave && mouse_hover {
                        mouse_hover = false;
                    }
                }
                _ => {}
            }
        }
        canvas.set_draw_color(grid::BACKGROUND_COLOR);
        canvas.clear();
        grid.draw_lines(&mut canvas);
        if mouse_hover && mouse_active {
            grid.draw_ghost_cursor(&mut canvas);
        }
        grid.draw_cursor(&mut canvas);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60))
    }
}
