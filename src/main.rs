use conways::ConwaysMap;
use macroquad::prelude::*;

mod conways;

const CELL_SIZE: f32 = 10.0;
const UPDATE_INTERVAL: f64 = 0.5;
const WIDTH: usize = 50;
const HEIGHT: usize = 50;

struct Game {
    conways: ConwaysMap,
    previous_time: f64,
    width: usize,
    height: usize,
    state: GameState
}

#[derive(PartialEq)]
enum GameState {
    Running,
    Paused
}

impl Game {
    pub fn new(width: usize, height: usize) -> Game {
        Game {
            conways: ConwaysMap::new(width, height),
            previous_time: get_time(),
            width,
            height,
            state: GameState::Running
        }
    }

    pub fn update(&mut self) {
        let time_of_last_frame = get_time();
        self.draw();

        // rendering time is too fast to be visually pleasing
        // update cells state in fixed periods of time
        if 
            time_of_last_frame - self.previous_time > UPDATE_INTERVAL
            && self.state == GameState::Running 
        {
            self.conways.tick();
            self.previous_time = time_of_last_frame;
        }

        if self.state == GameState::Paused {
            draw_text("Paused", 
            (WIDTH as f32 * CELL_SIZE) / 2.0 - 30.0, 
            (HEIGHT as f32 * CELL_SIZE) / 2.0, 
            25.0, 
            RED);
        }

        display_instructions();
    }

    fn draw(&self) {
        clear_background(BLACK);
        for x in  0..self.height {
            for y in 0..self.width {
                if self.conways.is_alive(x,y) {
                    draw_rectangle(
                        x as f32 * CELL_SIZE, 
                        y as f32 * CELL_SIZE, 
                        CELL_SIZE, 
                        CELL_SIZE, 
                        WHITE);
                }
            }
        }
    }

    // handles all interaction with the user
    pub fn user_interaction(&mut self) {
        if is_key_pressed(KeyCode::Space) {
            self.state = match self.state {
                GameState::Paused => GameState::Running,
                GameState::Running => GameState::Paused
            };
            
        }

        // Only can add or remove cells when paused
        // else it wouldn't be easy to form patterns
        if self.state == GameState::Paused {
            if is_mouse_button_pressed(MouseButton::Left) {
                self.set_alive();
            } else if is_mouse_button_pressed(MouseButton::Right) {
                self.set_dead();
            }
        }
    }

    fn set_dead(&mut self) {
        let (x, y) = get_mouse_grid_position();
        self.conways.set_dead(x, y);
    }
    
    fn set_alive(&mut self) {
        let (x, y) = get_mouse_grid_position();
        self.conways.set_alive(x, y);
    }

    pub fn set_blinker(&mut self) {
        self.conways.set_alive(1, 1);
        self.conways.set_alive(1, 0);
        self.conways.set_alive(1, 2);
    }
    
}

fn display_instructions() {
    draw_text("Play/Pause with Space", 10.0, 550.0, 22.5, BLUE);
    draw_text("Add cells when game paused with left click", 10.0, 580.0, 22.5, BLUE);
    draw_text("Remove cells when game paused with right click", 10.0, 610.0, 22.5, BLUE);
}

// get mouse position and get corresponding cell in the grid position
// cast it to usize
fn get_mouse_grid_position() -> (usize, usize) {
    let (mouse_x, mouse_y) = mouse_position();
    let x = mouse_x as usize / CELL_SIZE as usize;
    let y = mouse_y as usize / CELL_SIZE as usize;
    (x, y)
}

fn window_conf() -> Conf {
    Conf {
        fullscreen: false,
        window_title: "Conway's Game of Life".to_string(),
        window_width: CELL_SIZE as i32 * WIDTH as i32,
        window_height: CELL_SIZE as i32 * HEIGHT as i32 + 150,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new(WIDTH, HEIGHT);
    game.set_blinker();
    

    loop {
        game.user_interaction();
        game.update();
        next_frame().await;
    }
}
