use conways::Conways;
use macroquad::prelude::*;

mod conways;

const CELL_SIZE: f32 = 10.0;
const UPDATE_INTERVAL: f64 = 1.0;

struct Game {
    conways: Conways,
    previous_time: f64,
    width: usize,
    height: usize
}

impl Game {
    pub fn new(width: usize, height: usize) -> Game {
        Game {
            conways: Conways::new(width, height),
            previous_time: get_time(),
            width,
            height
        }
    }

    pub fn update(&mut self) {
        let time_of_last_frame = get_time();
        self.draw();
        if time_of_last_frame - self.previous_time > UPDATE_INTERVAL {
            self.conways.tick();
            self.previous_time = time_of_last_frame;
        }
    }
    fn draw(&self) {
        clear_background(BLACK);
        for x in  0..self.height {
            for y in 0..self.width {
                if self.conways.is_alive(x,y) {
                    draw_rectangle(x as f32 * CELL_SIZE, y as f32 * CELL_SIZE, CELL_SIZE, CELL_SIZE, WHITE);
                }
            }
        }
    }
}


#[macroquad::main("Conways")]
async fn main() {
    let width = 50;
    let height = 50;

    let mut game = Game::new(width, height);
    game.conways.set_alive(1, 1);
    game.conways.set_alive(1, 0);
    game.conways.set_alive(1, 2);
    loop {
        game.update();
        next_frame().await;
    }
}
