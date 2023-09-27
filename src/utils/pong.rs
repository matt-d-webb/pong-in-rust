extern crate piston_window;

use piston_window::*;

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
            ball = Ball::new();
            // TODO: determine the score based on which side the ball went off of.
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
        });
    }
}
