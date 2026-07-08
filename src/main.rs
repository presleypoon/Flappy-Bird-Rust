use macroquad::prelude::*;
use std::time::{Duration, Instant};

const FPS: f32 = 60.0;
const JUMP_FORCE: f32 = 20.0;
const GRAVITY: f32 = 2.0;
const X_POS: f32 = 30.0;
const BIRD_SIZE: f32 = 20.0;

struct Bird {
    // score: u128,
    y: f32,
    speed_y: f32,
}
impl Bird {
    fn jump(&mut self) {
        self.speed_y = JUMP_FORCE;
    }
    
    fn gravity(&mut self) {
        self.speed_y -= GRAVITY;
    }
    
    fn move_y(&mut self) {
        self.y += self.speed_y;
        if self.y >= 300.0 {
            panic!("Died, fly too high");
        }
        if self.y <= -300.0 {
            panic!("Died, fly too low");
        }
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
        y: 0.0,
        speed_y: 0.0,
    };

    loop {
        if is_key_down(KeyCode::Escape) {
            break;
        }

        let elapsed = last_tick.elapsed();
        last_tick = Instant::now();
        accumlator += elapsed;

        let mut space: bool = false;
        if is_key_down(KeyCode::Space) {
            space = true;
        }

        while accumlator >= tick_rate {
            game_logic(&mut bird, &mut space);
            accumlator -= tick_rate;
        }
        render(&bird);
        next_frame().await;
    }
}

fn game_logic(bird: &mut Bird, space: &mut bool) {
    if *space {
        bird.jump();
        *space = false;
    }
    bird.gravity();
    bird.move_y();
    println!("{}, {}", bird.y, bird.speed_y);
}

fn render(bird: &Bird) {
    clear_background(SKYBLUE);
    draw_rectangle(X_POS, 300.0 - bird.y, BIRD_SIZE, BIRD_SIZE, ORANGE);
}
