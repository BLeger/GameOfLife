extern crate minifb;

use minifb::{MouseButton, MouseMode, Key, WindowOptions, Window};
use std::time::{Instant};

const WIDTH: usize = 640;
const HEIGHT: usize = 640;

const CELL_SIZE: usize = 20;

const CELL_X: usize = WIDTH / CELL_SIZE;
const CELL_Y: usize = HEIGHT / CELL_SIZE;

const WHITE: u32 = 0xFFFFFF;
const GREY: u32  = 0xAAAAAA;
const BLACK: u32 = 0;

mod conway {
    pub struct Grid {
        foreground: Vec<bool>,
        background: Vec<bool>,
        width: usize,
        height: usize
    }

    impl Grid {
        pub fn new(width: usize, height: usize) -> Grid {
            Grid {
                foreground: vec![false; width * height],
                background: vec![false; width * height],
                width,
                height
            }
        }

        fn position_to_index(&self, x: usize, y: usize) -> usize {
            y * self.width + x
        }

        fn position_valid(&self, x: usize, y: usize) -> bool {
            x < self.width && y < self.height
        }

        pub fn is_cell_full(&self, x: usize, y: usize) -> bool {
            if !self.position_valid(x, y) {
                return false;
            }

            self.foreground[self.position_to_index(x, y)]
        }

        fn count_neighbors(&self, x: usize, y: usize) -> u32 {
            let mut neighbors = 0;
            
            if self.is_cell_full(x.wrapping_sub(1), y.wrapping_sub(1)) {
                neighbors += 1;
            }

            if self.is_cell_full(x.wrapping_sub(1), y) {
                neighbors += 1;
            }

            if self.is_cell_full(x.wrapping_sub(1), y.wrapping_add(1)) {
                neighbors += 1;
            }

            if self.is_cell_full(x, y.wrapping_sub(1)) {
                neighbors += 1;
            }

            if self.is_cell_full(x, y.wrapping_add(1)) {
                neighbors += 1;
            }

            if self.is_cell_full(x.wrapping_add(1), y.wrapping_sub(1)) {
                neighbors += 1;
            }

            if self.is_cell_full(x.wrapping_add(1), y) {
                neighbors += 1;
            }

            if self.is_cell_full(x.wrapping_add(1), y.wrapping_add(1)) {
                neighbors += 1;
            }

            neighbors
        }

        pub fn update(&mut self) {
            for x in 0..self.width {
                for y in 0..self.height {
                    let index = self.position_to_index(x, y);
                    let neighbors = self.count_neighbors(x, y);

                    let state = self.foreground[index];
                    let mut new_state = false;

                    if state {
                        if neighbors == 2 || neighbors == 3 {
                            new_state = true;
                        }
                    } else if neighbors == 3 {
                        new_state = true;
                    }

                    self.background[index] = new_state;
                }
            }

            std::mem::swap(&mut self.foreground, &mut self.background);
        }

        pub fn set_cell(&mut self, x: usize, y: usize, value: bool) {
            if self.position_valid(x, y) {
                let index = self.position_to_index(x, y);
                self.foreground[index] = value;
            }
        }
    }
}

use conway::Grid;

fn main() {
    let mut grid = Grid::new(CELL_X, CELL_Y);
    let mut pause = true;
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("Conway's game of life",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut space_pressed = false;
    let mut clock = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {

        if window.is_key_down(Key::Space) {
            if !space_pressed {
                pause = !pause;
                space_pressed = true;
                
                match pause {
                    true => println!("Game paused"),
                    false => {
                        println!("Game resumed");
                        clock = Instant::now();
                    } 
                };
            }
        } else {
            space_pressed = false;
        }

        window.get_mouse_pos(MouseMode::Discard).map(|(pixel_x, pixel_y)| {
            let x = pixel_x as usize / CELL_SIZE;
            let y = pixel_y as usize / CELL_SIZE;

            if window.get_mouse_down(MouseButton::Left) {
                grid.set_cell(x, y, true);
            } else if window.get_mouse_down(MouseButton::Right) {
                grid.set_cell(x, y, false);
            }
        });

        if !pause && clock.elapsed().as_millis() >= 500 {
            grid.update();
            clock = Instant::now();
        }

        // Pixel buffer
        for (i, color) in buffer.iter_mut().enumerate() {
            let pixel_x = i % WIDTH;
            let pixel_y = i / HEIGHT;

            let x = pixel_x / CELL_SIZE;
            let y = pixel_y / CELL_SIZE;

            // Display the grid
            if pixel_x % CELL_SIZE == 0 || pixel_y % CELL_SIZE == 0 {
                *color = GREY;
            } else {
                // Fill the cell or not
                if let true = grid.is_cell_full(x, y) {
                    *color = WHITE;
                } else {
                    *color = BLACK;
                }
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer).unwrap();

    }
} 