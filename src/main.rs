use conways::Conways;
use macroquad::prelude::*;

mod conways;

const CELL_SIZE: f32 = 50.0;
const UPDATE_INTERVAL: f64 = 1.0;

fn draw(conways: &Conways, width: usize, height: usize) {
    clear_background(BLACK);
    for x in  0..height {
        for y in 0..width {
            if conways.is_alive(x,y) {
                draw_rectangle(x as f32 * CELL_SIZE, y as f32 * CELL_SIZE, CELL_SIZE, CELL_SIZE, WHITE);
            }
        }
    }
}

#[macroquad::main("Conways")]
async fn main() {
    let mut conway = Conways::new(6, 6);
    conway.set_alive(1,0);
    conway.set_alive(1,1);
    conway.set_alive(1,2);

    let width = 50;
    let height = 50;

    let mut previous_time = get_time();
    draw(&conway, width, height);
    loop {
        let time_of_last_frame = get_time();
        if time_of_last_frame - previous_time > UPDATE_INTERVAL {
            draw(&conway, width, height);
            conway.tick();
            next_frame().await;
            previous_time = time_of_last_frame;
        }
    }
}
