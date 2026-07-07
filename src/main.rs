use macroquad::prelude::*;
use std::time::{Duration, Instant};

const FPS: f32 = 60.0;
const JUMP_FORCE: u8 = 10;
const GRAVITY: u8 = 2;

struct Bird {
    y: u8,
    speed_y: u8,
    score: u128,
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

#[macroquad::main("Flappy Bird")]
async fn main() {
    let tick_rate: Duration = Duration::from_secs_f32(1.0 / FPS);
    let mut last_tick: Instant = Instant::now();
    let mut accumlator: Duration = Duration::ZERO;

    let mut bird: Bird = Bird {
        score: 0,
        y: 0,
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
        render();
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

fn render() {
    todo!();
}
