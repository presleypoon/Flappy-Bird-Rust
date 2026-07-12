use macroquad::prelude::*;
use std::time::{Duration, Instant};

const FPS: f32 = 60.0;
const JUMP_FORCE: f32 = 10.0;
const GRAVITY: f32 = 1.0;
const X_POS: f32 = 30.0;
const BIRD_SIZE: f32 = 20.0;
const PIPE_DIST: usize = 200;

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

    fn update_score(&mut self, pipe_offset: usize) -> bool {
        if pipe_offset == 870 % PIPE_DIST {
            if self.score.checked_add(10).is_none() {
                println!("You spent too much time on this game!");
                return true;
            }
            self.score += 10;
        }
        false
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

    let mut pipes: [Option<u32>; 15] = [None; 15];
    let mut pipe_offset: usize = 0;

    pipe(&mut pipes, &mut pipe_offset);
    render(&bird, pipes, pipe_offset);
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

        render(&bird, pipes, pipe_offset);

        next_frame().await;
    }
}

fn game_logic(
    bird: &mut Bird,
    space: &mut bool,
    pipes: &mut [Option<u32>; 15],
    pipe_offset: &mut usize,
) -> bool {
    *pipe_offset += 1;
    *pipe_offset %= PIPE_DIST;

    if *space {
        bird.jump();
        *space = false;
    }
    bird.gravity();
    pipe(pipes, pipe_offset);
    bird.update_score(*pipe_offset) || bird.move_y()
}

fn render(bird: &Bird, pipes: [Option<u32>; 15], pipe_offset: usize) {
    clear_background(SKYBLUE);
    draw_rectangle(X_POS, 300.0 - bird.y, BIRD_SIZE, BIRD_SIZE, ORANGE);

    for (i, pipe) in pipes.iter().enumerate() {
        if let Some(pipe_unwraped) = pipe {
            draw_rectangle(
                900.0 - ((pipe_offset + i * PIPE_DIST) as f32),
                -100.0 - *pipe_unwraped as f32,
                25.0,
                500.0,
                GREEN,
            );
            draw_rectangle(
                900.0 - ((pipe_offset + i * PIPE_DIST) as f32),
                600.0 - *pipe_unwraped as f32,
                25.0,
                500.0,
                GREEN,
            );
        }
    }

    let ign: String = number_to_ign(bird.score);
    // let ign: String = "12345678990.".to_string();
    // let ign: String = "KMBTQSONDUaixp".to_string();
    for (i, char) in ign.chars().enumerate() {
        print_char(char, i);
    }
}

fn pipe(pipes: &mut [Option<u32>; 15], pipe_offset: &mut usize) {
    if *pipe_offset == 0 {
        *pipes = [
            None, pipes[0], pipes[1], pipes[2], pipes[3], pipes[4], pipes[5], pipes[6], pipes[7],
            pipes[8], pipes[9], pipes[10], pipes[11], pipes[12], pipes[13],
        ];
    }

    for (i, pipe) in pipes.clone().iter().enumerate() {
        if pipe.is_none() {
            pipes[i] = Some(::rand::random_range(0..300));
        }
    }
}

#[allow(dead_code)]
fn number_to_ign(number: u128) -> String {
    if number < 1000 {
        return format!("{:>4}  ", number);
    }

    let suffixes: [&str; 12] = [
        "K ", "M ", "B ", "T ", "Qa", "Qi", "Sx", "Sp", "O ", "N ", "D ", "U ",
    ];

    let f_num: f32 = number as f32;
    let exp_i_div_3: usize = (f_num.log10() / 3.0) as usize;
    let coef: f32 = f_num / 1000_f32.powi(exp_i_div_3 as i32);
    let suffix: &str = suffixes[exp_i_div_3 - 1];

    if coef >= 100.0 {
        format!("{:>4.0}{}", coef, suffix)
    } else if coef >= 10.0 {
        format!("{:>4.1}{}", coef, suffix)
    } else {
        format!("{:>4.2}{}", coef, suffix)
    }
}

fn print_char(char: char, i: usize) {
    match char {
        '0' => draw_segs(&['a', 'b', 'c', 'f', 'g', 'j', 'k', 'n', 'o', 'p'], i),
        '1' => draw_segs(&['g', 'n'], i),
        '2' => draw_segs(&['a', 'b', 'g', 'h', 'i', 'j', 'o', 'p'], i),
        '3' => draw_segs(&['a', 'b', 'g', 'h', 'i', 'n', 'o', 'p'], i),
        '4' => draw_segs(&['c', 'g', 'h', 'i', 'n'], i),
        '5' => draw_segs(&['a', 'b', 'c', 'h', 'i', 'n', 'o', 'p'], i),
        '6' => draw_segs(&['a', 'b', 'c', 'h', 'i', 'j', 'n', 'o', 'p'], i),
        '7' => draw_segs(&['a', 'b', 'g', 'n'], i),
        '8' => draw_segs(&['a', 'b', 'c', 'g', 'h', 'i', 'j', 'n', 'o', 'p'], i),
        '9' => draw_segs(&['a', 'b', 'c', 'g', 'h', 'i', 'n', 'o', 'p'], i),
        '.' => draw_segs(&['q'], i),
        'K' => draw_segs(&['c', 'f', 'h', 'j', 'm'], i),
        'M' => draw_segs(&['c', 'd', 'f', 'g', 'j', 'n'], i),
        'B' => draw_segs(&['a', 'b', 'e', 'g', 'i', 'l', 'n', 'o', 'p'], i),
        'T' => draw_segs(&['a', 'b', 'e', 'l'], i),
        'Q' => draw_segs(&['a', 'b', 'c', 'g', 'j', 'n', 'm', 'o', 'p'], i),
        'S' => draw_segs(&['a', 'b', 'd', 'm', 'o', 'p'], i),
        'O' => draw_segs(&['a', 'b', 'c', 'g', 'j', 'n', 'o', 'p'], i),
        'N' => draw_segs(&['c', 'd', 'g', 'j', 'm', 'n'], i),
        'D' => draw_segs(&['a', 'b', 'e', 'g', 'l', 'n', 'o', 'p'], i),
        'U' => draw_segs(&['c', 'g', 'j', 'n', 'o', 'p'], i),
        'a' => draw_segs(&['h', 'j', 'l', 'o', 'p'], i),
        'i' => draw_segs(&['a', 'h', 'l', 'o', 'p'], i),
        'x' => draw_segs(&['h', 'i', 'k', 'm'], i),
        'p' => draw_segs(&['a', 'c', 'e', 'h', 'j'], i),
        ' ' => {}
        _ => {
            unreachable!("Invalid Char");
        }
    }
}

fn draw_segs(letters: &[char], offset: usize) {
    const X: [f32; 4] = [7.5, 20.0, 32.5, 39.0];
    const Y: [f32; 3] = [7.5, 32.5, 57.5];
    const CHAR_DIST: f32 = 40.0;

    for letter in letters {
        match letter {
            'a' => draw_line(
                X[0] + offset as f32 * CHAR_DIST,
                Y[0],
                X[1] + offset as f32 * CHAR_DIST,
                X[0],
                5.0,
                BLACK,
            ),
            'b' => draw_line(
                X[1] + offset as f32 * CHAR_DIST,
                Y[0],
                X[2] + offset as f32 * CHAR_DIST,
                Y[0],
                5.0,
                BLACK,
            ),
            'c' => draw_line(
                X[0] + offset as f32 * CHAR_DIST,
                Y[0],
                X[0] + offset as f32 * CHAR_DIST,
                Y[1],
                5.0,
                BLACK,
            ),
            'd' => draw_line(
                X[0] + offset as f32 * CHAR_DIST,
                Y[0],
                X[1] + offset as f32 * CHAR_DIST,
                Y[1],
                5.0,
                BLACK,
            ),
            'e' => draw_line(
                X[1] + offset as f32 * CHAR_DIST,
                Y[0],
                X[1] + offset as f32 * CHAR_DIST,
                Y[1],
                5.0,
                BLACK,
            ),
            'f' => draw_line(
                X[2] + offset as f32 * CHAR_DIST,
                Y[0],
                X[1] + offset as f32 * CHAR_DIST,
                Y[1],
                5.0,
                BLACK,
            ),
            'g' => draw_line(
                X[2] + offset as f32 * CHAR_DIST,
                Y[0],
                X[2] + offset as f32 * CHAR_DIST,
                Y[1],
                5.0,
                BLACK,
            ),
            'h' => draw_line(
                X[0] + offset as f32 * CHAR_DIST,
                Y[1],
                X[1] + offset as f32 * CHAR_DIST,
                Y[1],
                5.0,
                BLACK,
            ),
            'i' => draw_line(
                X[1] + offset as f32 * CHAR_DIST,
                Y[1],
                X[2] + offset as f32 * CHAR_DIST,
                Y[1],
                5.0,
                BLACK,
            ),
            'j' => draw_line(
                X[0] + offset as f32 * CHAR_DIST,
                Y[1],
                X[0] + offset as f32 * CHAR_DIST,
                Y[2],
                5.0,
                BLACK,
            ),
            'k' => draw_line(
                X[1] + offset as f32 * CHAR_DIST,
                Y[1],
                X[0] + offset as f32 * CHAR_DIST,
                Y[2],
                5.0,
                BLACK,
            ),
            'l' => draw_line(
                X[1] + offset as f32 * CHAR_DIST,
                Y[1],
                X[1] + offset as f32 * CHAR_DIST,
                Y[2],
                5.0,
                BLACK,
            ),
            'm' => draw_line(
                X[1] + offset as f32 * CHAR_DIST,
                Y[1],
                X[2] + offset as f32 * CHAR_DIST,
                Y[2],
                5.0,
                BLACK,
            ),
            'n' => draw_line(
                X[2] + offset as f32 * CHAR_DIST,
                Y[1],
                X[2] + offset as f32 * CHAR_DIST,
                Y[2],
                5.0,
                BLACK,
            ),
            'o' => draw_line(
                X[0] + offset as f32 * CHAR_DIST,
                Y[2],
                X[1] + offset as f32 * CHAR_DIST,
                Y[2],
                5.0,
                BLACK,
            ),
            'p' => draw_line(
                X[1] + offset as f32 * CHAR_DIST,
                Y[2],
                X[2] + offset as f32 * CHAR_DIST,
                Y[2],
                5.0,
                BLACK,
            ),
            'q' => draw_line(
                X[2] + offset as f32 * CHAR_DIST,
                Y[2],
                X[3] + offset as f32 * CHAR_DIST,
                Y[2],
                5.0,
                BLACK,
            ),
            _ => {
                unreachable!("invalid letter");
            }
        }
    }
}
