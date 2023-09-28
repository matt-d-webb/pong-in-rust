extern crate piston_window;

use piston_window::*;


// use gfx_device_gl::{Factory, Resources, CommandBuffer};

const WIDTH: f64 = 640.0;
const HEIGHT: f64 = 480.0;

struct Ball {
    x: f64,
    y: f64,
    velocity_x: f64,
    velocity_y: f64,
}

impl Ball {
    fn new() -> Self {
        Ball {
            x: WIDTH / 2.0,
            y: HEIGHT / 2.0,
            velocity_x: 1.0,
            velocity_y: 1.0,
        }
    }
    fn update(&mut self) {
        self.x += self.velocity_x;
        self.y += self.velocity_y;
        if self.y <= 0.0 || self.y >= HEIGHT {
            self.velocity_y = -self.velocity_y;
        }
    }
}

struct Paddle {
    x: f64,
    y: f64,
    velocity: f64,
}

impl Paddle {
    fn new(x: f64) -> Self {
        Paddle {
            x,
            y: HEIGHT / 2.0,
            velocity: 0.0,
        }
    }

    fn update(&mut self) {
        self.y += self.velocity;

        if self.y < 0.0 {
            self.y = 0.0;
        } else if self.y > HEIGHT - 60.0 {
            self.y = HEIGHT - 60.0;
        }
    }
}

pub fn run() {
    let mut window: PistonWindow = WindowSettings::new("Pong", [WIDTH as u32, HEIGHT as u32])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut ball: Ball = Ball::new();
    let mut left_paddle: Paddle = Paddle::new(10.0);
    let mut right_paddle: Paddle = Paddle::new(WIDTH - 30.0);
    let mut side_a_score: i32 = 0;
    let mut side_b_score: i32 = 0;

    let width = 640;
    let height = 480;

    let assets = find_folder::Search::ParentsThenKids(3, 3)
    .for_folder("assets").unwrap();

    let texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into()
    };
    let texture_settings = TextureSettings::new();
    let mut glyphs = Glyphs::new(&assets.join("FiraSans-Regular.ttf"), texture_context, texture_settings).unwrap();


    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Up => right_paddle.velocity = -2.0,
                Key::Down => right_paddle.velocity = 2.0,
                Key::W => left_paddle.velocity = -2.0,
                Key::S => left_paddle.velocity = 2.0,
                _ => {}
            }
        }

        if let Some(Button::Keyboard(key)) = event.release_args() {
            match key {
                Key::Up | Key::Down => right_paddle.velocity = 0.0,
                Key::W | Key::S => left_paddle.velocity = 0.0,
                _ => {}
            }
        }

        ball.update();
        left_paddle.update();
        right_paddle.update();

        // Ball collision with paddles
        if ball.x < left_paddle.x + 20.0
            && ball.x > left_paddle.x
            && ball.y > left_paddle.y
            && ball.y < left_paddle.y + 60.0
            || ball.x > right_paddle.x - 20.0
                && ball.x < right_paddle.x + 20.0
                && ball.y > right_paddle.y
                && ball.y < right_paddle.y + 60.0
        {
            ball.velocity_x = -ball.velocity_x;
        }

        // Reset ball if it goes out of bounds
        if ball.x < 0.0 || ball.x > WIDTH {
            if ball.x < 0.0 {
                side_b_score += 1; // Right player scores if ball goes out on the left side
            } else {
                side_a_score += 1; // Left player scores if ball goes out on the right side
            }
            ball = Ball::new();
        }

        window.draw_2d(&event, |c: Context, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            rectangle(
                [1.0, 0.0, 0.0, 1.0],
                [left_paddle.x, left_paddle.y, 20.0, 60.0],
                c.transform,
                g,
            );
            rectangle(
                [0.0, 0.0, 1.0, 1.0],
                [right_paddle.x, right_paddle.y, 20.0, 60.0],
                c.transform,
                g,
            );
            ellipse(
                [1.0, 1.0, 1.0, 1.0],
                [ball.x, ball.y, 15.0, 15.0],
                c.transform,
                g,
            );

            // Scoreboard background (at the bottom of the screen)
            rectangle(
                [0.5, 0.5, 0.5, 1.0],
                [0.0, height as f64 - 50.0, width as f64, 50.0],
                c.transform,
                g,
            );

            // Display score for side A
            let score_a_text = format!("Side A: {}", side_a_score);
            text::Text::new_color([1.0, 1.0, 1.0, 1.0], 20)
                .draw(
                    &score_a_text,
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(WIDTH - 120.0, HEIGHT - 20.0),
                    g,
                )
                .unwrap();

            // Display score for side B
            let score_b_text = format!("Side B: {}", side_b_score);
            text::Text::new_color([1.0, 1.0, 1.0, 1.0], 20)
                .draw(
                    &score_b_text,
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(WIDTH - 120.0, HEIGHT - 20.0),
                    g,
                )
                .unwrap();
        });
    }
}
