use macroquad::prelude::*;

// constants for game objects 
const PADDLE_WIDTH: f32 = 100.0;
const PADDLE_HEIGHT: f32 = 20.0;
const BALL_SIZE: f32 = 10.0;
const BRANCH_WIDTH: f32 = 300.0;
const BRANCH_HEIGHT: f32 = 30.0;

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

enum GameState {
    Playing,
    Won,
}

#[macroquad::main("Stickbreaker")]
async fn main() {
    // initial paddle object 
    let mut paddle = GameObject {
        rect: Rect::new(
            screen_width() / 2.0 - PADDLE_WIDTH / 2.0, // center paddle horizontally 
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
        color: WHITE,
    };

    let mut branch = GameObject {
        rect: Rect::new(
            screen_width() / 2.0 - BRANCH_WIDTH / 2.0,
            100.0,
            BRANCH_WIDTH,
            BRANCH_HEIGHT,
        ),
        color: GREEN,
    };

    let mut game_state = GameState::Playing;

    loop {
        clear_background(BLACK);

        match game_state {
            GameState::Playing => {
                // update paddle position
                paddle.rect.x = mouse_position().0 - PADDLE_WIDTH / 2.0;
                paddle.rect.x = paddle.rect.x.clamp(0.0, screen_width() - PADDLE_WIDTH);

                // update ball position
                ball.pos += ball.vel * get_frame_time();

                // ball collision with walls
                if ball.pos.x < 0.0 || ball.pos.x > screen_width() {
                    ball.vel.x = -ball.vel.x;
                }
                if ball.pos.y < 0.0 {
                    ball.vel.y = -ball.vel.y;
                }

                // ball collision with paddle
                if ball.pos.y + ball.size > paddle.rect.y
                    && ball.pos.x > paddle.rect.x
                    && ball.pos.x < paddle.rect.x + paddle.rect.w
                {
                    ball.vel.y = -ball.vel.y;
                }

                // ball collision with branch
                if ball.pos.y - ball.size < branch.rect.y + branch.rect.h
                    && ball.pos.y + ball.size > branch.rect.y
                    && ball.pos.x > branch.rect.x
                    && ball.pos.x < branch.rect.x + branch.rect.w
                {
                    ball.vel.y = -ball.vel.y;
                    branch.rect.w -= 30.0; // reduce branch width when hit
                    if branch.rect.w <= 0.0 {
                        game_state = GameState::Won;
                    }
                }

                // draw game objects
                draw_rectangle(paddle.rect.x, paddle.rect.y, paddle.rect.w, paddle.rect.h, paddle.color);
                draw_circle(ball.pos.x, ball.pos.y, ball.size, ball.color);
                draw_rectangle(branch.rect.x, branch.rect.y, branch.rect.w, branch.rect.h, branch.color);
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
                    WHITE,
                );
            }
        }

        next_frame().await
    }
}