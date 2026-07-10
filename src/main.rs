use macroquad::prelude::*;
use macroquad_text::Fonts;
use std::time::{Duration, Instant};

const FPS: f32 = 60.0;
const JUMP_FORCE: f32 = 20.0;
const GRAVITY: f32 = 2.0;
const X_POS: f32 = 30.0;
const BIRD_SIZE: f32 = 20.0;

struct Bird {
    score: u128,
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

    fn move_y(&mut self) -> bool {
        self.y += self.speed_y;
        if self.y >= 300.0 {
            println!("Died, fly too high");
            true
        } else if self.y <= -300.0 {
            println!("Died, fly too low");
            true
        } else {
            false
        }
    }
}

fn window() -> Conf {
    Conf {
        window_title: "Flappy Bird".to_string(),
        window_width: 1000,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

const NOTO_SANS: &[u8] = include_bytes!("../assets/fonts/NotoSans-Regular.ttf");

#[macroquad::main(window)]
async fn main() {
    let tick_rate: Duration = Duration::from_secs_f32(1.0 / FPS);
    let mut last_tick: Instant = Instant::now();
    let mut accumlator: Duration = Duration::ZERO;
    let mut running: bool = false;

    let mut bird: Bird = Bird {
        score: 0,
        y: 0.0,
        speed_y: 0.0,
    };

    let mut fonts: Fonts<'_> = Fonts::default();
    fonts.load_font_from_bytes("Noto Sans", NOTO_SANS).unwrap();

    let mut pipes: [Option<u32>; 10] = [None; 10];
    let mut pipe_offset: usize = 0;

    println!("Init done");

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

        if is_any_key_down() {
            running = true;
        }

        if running {
            while accumlator >= tick_rate {
                if game_logic(&mut bird, &mut space, &mut pipes, &mut pipe_offset) {
                    return;
                }
                accumlator -= tick_rate;
            }
        } else {
            accumlator = Duration::ZERO;
        }
        render(&bird, &mut fonts, pipes, pipe_offset);
        next_frame().await;
    }
}

fn game_logic(
    bird: &mut Bird,
    space: &mut bool,
    pipes: &mut [Option<u32>; 10],
    pipe_offset: &mut usize,
) -> bool {
    *pipe_offset += 1;
    *pipe_offset %= 100;

    if *space {
        bird.jump();
        *space = false;
    }
    bird.gravity();
    pipe(pipes, pipe_offset);
    println!("{}, {}", bird.y, bird.speed_y);
    bird.move_y()
}

fn render(bird: &Bird, fonts: &mut Fonts<'_>, pipes: [Option<u32>; 10], pipe_offset: usize) {
    clear_background(SKYBLUE);
    draw_rectangle(X_POS, 300.0 - bird.y, BIRD_SIZE, BIRD_SIZE, ORANGE);

    for (i, pipe) in pipes.iter().enumerate() {
        if let Some(pipe_unwraped) = pipe {
            draw_rectangle(
                (pipe_offset + i * 100) as f32,
                *pipe_unwraped as f32 + 500.0,
                10.0,
                500.0,
                GREEN,
            );
            draw_rectangle(
                (pipe_offset + i * 100) as f32,
                *pipe_unwraped as f32 - 100.0,
                10.0,
                500.0,
                GREEN,
            );
        }
    }

    fonts.draw_text(&bird.score.to_string(), 10.0, 0.0, 12, BLACK);
}

#[allow(dead_code)]
fn pipe(pipes: &mut [Option<u32>; 10], pipe_offset: &mut usize) {
    let mut none: Vec<usize> = vec![];
    for (i, pipe) in pipes.iter().enumerate() {
        if pipe.is_none() {
            none.push(i);
            break;
        }
    }

    if *pipe_offset == 0 {
        *pipes = pop_pipe(*pipes);
    }

    pipes[9] = Some(rand::rand() / u32::MAX * 500);
}

fn pop_pipe(pipe: [Option<u32>; 10]) -> [Option<u32>; 10] {
    [
        pipe[1], pipe[2], pipe[3], pipe[4], pipe[5], pipe[6], pipe[7], pipe[8], pipe[9], None,
    ]
}
