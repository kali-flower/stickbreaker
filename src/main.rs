use macroquad::prelude::*;

const PADDLE_WIDTH: f32 = 100.0;
const PADDLE_HEIGHT: f32 = 20.0;
const BALL_SIZE: f32 = 10.0;
const BRANCH_WIDTH: i32 = 60;
const BRANCH_HEIGHT: i32 = 10;
const PIXEL_SIZE: f32 = 10.0;
const BREAK_THRESHOLD: i32 = 8;

struct GameObject {
    rect: Rect,
    color: Color,
}

struct Ball {
    pos: Vec2,
    vel: Vec2,
    size: f32,
    color: Color,
}

struct Branch {
    pixels: Vec<Vec<bool>>,
    pos: Vec2,
}

enum GameState {
    Playing,
    Won,
}

#[macroquad::main("Stickbreaker")]
async fn main() {
    let mut paddle = GameObject {
        rect: Rect::new(
            screen_width() / 2.0 - PADDLE_WIDTH / 2.0,
            screen_height() - 50.0,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        ),
        color: BLUE,
    };

    let mut ball = Ball {
        pos: vec2(screen_width() / 2.0, screen_height() / 2.0),
        vel: vec2(200.0, -200.0),
        size: BALL_SIZE,
        color: RED,
    };

    let mut branch = Branch {
        pixels: vec![vec![true; BRANCH_WIDTH as usize]; BRANCH_HEIGHT as usize],
        pos: vec2(50.0, 100.0),
    };

    let mut game_state = GameState::Playing;

    loop {
        clear_background(WHITE);

        match game_state {
            GameState::Playing => {
                // update paddle position
                paddle.rect.x = mouse_position().0 - PADDLE_WIDTH / 2.0;
                paddle.rect.x = paddle.rect.x.clamp(0.0, screen_width() - PADDLE_WIDTH);

                // update ball position
                let new_pos = ball.pos + ball.vel * get_frame_time();

                // ball collision with walls
                if new_pos.x - ball.size < 0.0 || new_pos.x + ball.size > screen_width() {
                    ball.vel.x = -ball.vel.x;
                }
                if new_pos.y - ball.size < 0.0 {
                    ball.vel.y = -ball.vel.y;
                }

                // ball collision with paddle
                if new_pos.y + ball.size > paddle.rect.y
                    && new_pos.x > paddle.rect.x
                    && new_pos.x < paddle.rect.x + paddle.rect.w
                {
                    ball.vel.y = -ball.vel.y.abs();
                    let paddle_center = paddle.rect.x + paddle.rect.w / 2.0;
                    let distance_from_center = new_pos.x - paddle_center;
                    ball.vel.x = distance_from_center * 5.0;
                }

                // ball collision with branch
                let mut check_collision = |x: f32, y: f32| {
                    let ball_branch_x = ((x - branch.pos.x) / PIXEL_SIZE) as i32;
                    let ball_branch_y = ((y - branch.pos.y) / PIXEL_SIZE) as i32;
                    if ball_branch_x >= 0 && ball_branch_x < BRANCH_WIDTH && ball_branch_y >= 0 && ball_branch_y < BRANCH_HEIGHT {
                        if branch.pixels[ball_branch_y as usize][ball_branch_x as usize] {
                            branch.pixels[ball_branch_y as usize][ball_branch_x as usize] = false;
                            return true;
                        }
                    }
                    false
                };

                let top = new_pos.y - ball.size;
                let bottom = new_pos.y + ball.size;
                let left = new_pos.x - ball.size;
                let right = new_pos.x + ball.size;

                if check_collision(new_pos.x, top) || check_collision(new_pos.x, bottom) {
                    ball.vel.y = -ball.vel.y;
                }
                if check_collision(left, new_pos.y) || check_collision(right, new_pos.y) {
                    ball.vel.x = -ball.vel.x;
                }

                ball.pos = new_pos;

                // check if branch is broken (8 pixels in a column are destroyed)
                let mut branch_broken = false;
                for x in 0..BRANCH_WIDTH {
                    let broken_pixels = branch.pixels.iter().filter(|row| !row[x as usize]).count();
                    if broken_pixels >= BREAK_THRESHOLD as usize {
                        branch_broken = true;
                        break;
                    }
                }

                if branch_broken {
                    game_state = GameState::Won;
                }

                // draw game objects
                draw_rectangle(paddle.rect.x, paddle.rect.y, paddle.rect.w, paddle.rect.h, paddle.color);
                draw_circle(ball.pos.x, ball.pos.y, ball.size, ball.color);

                // draw branch pixels
                for (y, row) in branch.pixels.iter().enumerate() {
                    for (x, &pixel) in row.iter().enumerate() {
                        if pixel {
                            draw_rectangle(
                                branch.pos.x + x as f32 * PIXEL_SIZE,
                                branch.pos.y + y as f32 * PIXEL_SIZE,
                                PIXEL_SIZE,
                                PIXEL_SIZE,
                                BLACK,
                            );
                        }
                    }
                }
            }
            GameState::Won => {
                let text = "You Won!";
                let text_size = 40.0;
                let text_dims = measure_text(text, None, text_size as u16, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dims.width / 2.0,
                    screen_height() / 2.0,
                    text_size,
                    BLACK,
                );
            }
        }

        next_frame().await
    }
}