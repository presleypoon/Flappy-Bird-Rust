use macroquad::prelude::*;
use std::time::{Duration, Instant};

const FPS: f32 = 60.0;
const JUMP_FORCE: u16 = 10;
const GRAVITY: u16 = 2;
const X_POS: u16 = 30;
const BIRD_SIZE: u8 = 10;

struct Bird {
    // score: u128,
    y: u16,
    speed_y: u16,
}
impl Bird {
    fn jump(&mut self) {
        self.speed_y += JUMP_FORCE;
    }

    fn gravity(&mut self) {
        self.speed_y -= GRAVITY;
    }

    fn move_y(&mut self) {
        self.y += self.speed_y;
    }
}

fn window() -> Conf {
    Conf {
        window_title: "Flappy Bird".to_string(),
        window_width: 400,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window)]
async fn main() {
    let tick_rate: Duration = Duration::from_secs_f32(1.0 / FPS);
    let mut last_tick: Instant = Instant::now();
    let mut accumlator: Duration = Duration::ZERO;

    let mut bird: Bird = Bird {
        // score: 0,
        y: 300,
        speed_y: 0,
    };

    loop {
        let elapsed = last_tick.elapsed();
        last_tick = Instant::now();
        accumlator += elapsed;

        while accumlator >= tick_rate {
            game_logic(&mut bird);
            accumlator -= tick_rate;
        }
        render(&bird);
        next_frame().await;
    }
}

fn game_logic(bird: &mut Bird) {
    if is_key_pressed(KeyCode::Space) {
        bird.jump();
    }
    bird.gravity();
    bird.move_y();
}

fn render(bird: &Bird) {
    draw_rectangle(X_POS as f32, bird.y as f32, BIRD_SIZE as f32, BIRD_SIZE as f32, ORANGE);    
}
