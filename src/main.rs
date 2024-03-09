use macroquad::prelude::*;

const PADDLE_SPEED: f32 = 2.0;
const BALL_BASE_SPEED: f32 = 0.3; // Initial ball speed
const BALL_SPEED_INCREMENT: f32 = 0.025; // Speed increment when hitting a paddle

struct MainState {
    top_paddle: Rect,
    bottom_paddle: Rect,
    ball: Rect,
    ball_vel: Vec2,
    top_score: usize,
    bottom_score: usize,
    ball_speed: f32,
}

impl MainState {
    fn new() -> Self {
        MainState {
            top_paddle: Rect::new(screen_width() / 2.0, 10.0, 200.0, 10.0),
            bottom_paddle: Rect::new(screen_width() / 2.0, screen_height() - 30.0, 200.0, 10.0),
            ball: Rect::new(screen_width() / 2.0, screen_height() / 2.0, 10.0, 10.0),
            ball_vel: Vec2::new(2.0, 5.0),
            top_score: 0,
            bottom_score: 0,
            ball_speed: BALL_BASE_SPEED,
        }
    }

    fn reset_ball(&mut self) {
        self.ball_vel.x *= -1.0;
        self.ball_vel.y *= -1.0;
        self.ball.move_to(Vec2::new(screen_width() / 2.0, screen_height() / 2.0));
        self.ball_speed = BALL_BASE_SPEED;
    }

    fn update(&mut self) {
        if is_key_down(KeyCode::Left) {
            self.top_paddle.x -= PADDLE_SPEED;
        }
        if is_key_down(KeyCode::Right) {
            self.top_paddle.x += PADDLE_SPEED;
        }
        
        // AI paddle movement
        let ai_paddle_target_x = self.ball.x - self.bottom_paddle.w / 2.0;
        let ai_paddle_speed = 25.0; // Adjust the AI paddle speed as needed

        // Adjust AI paddle position based on target position and speed
        if ai_paddle_target_x < self.bottom_paddle.x {
            self.bottom_paddle.x -= ai_paddle_speed;
        } else if ai_paddle_target_x > self.bottom_paddle.right() {
            self.bottom_paddle.x += ai_paddle_speed;
        }
        
        

        // if is_key_down(KeyCode::Left) {
        //     self.bottom_paddle.x -= PADDLE_SPEED;
        // }
        // if is_key_down(KeyCode::Right) {
        //     self.bottom_paddle.x += PADDLE_SPEED;
        // }

        self.ball.x += self.ball_vel.x * self.ball_speed; // Multiply velocity by speed
        self.ball.y += self.ball_vel.y * self.ball_speed; // Multiply velocity by speed

        if self.ball.overlaps(&self.bottom_paddle) 
        || self.ball.overlaps(&self.top_paddle) {
            self.ball_vel.y *= -1.0;
            self.ball_speed += BALL_SPEED_INCREMENT;
        }

        if self.ball.left() <= 0.0 
        || self.ball.right() >= screen_width() {
            self.ball_vel.x *= -1.0;
        }

        if self.ball.top() <= 0.0 {
            self.bottom_score += 1;
            self.reset_ball();
        }
        if self.ball.bottom() >= screen_height() {
            self.top_score += 1;
            self.reset_ball();
        }
    }

    fn draw_rect(&self, rect: &Rect) {
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            WHITE,
        )
    }

    fn draw(&self) {
        self.draw_rect(&self.top_paddle);
        self.draw_rect(&self.bottom_paddle);
        self.draw_rect(&self.ball);

        draw_text(&self.top_score.to_string(), 20.0, 20.0, 40.0, WHITE);
        draw_text(&self.bottom_score.to_string(), 20.0, screen_height() - 20.0, 40.0, WHITE);
    }
}

#[macroquad::main("Pong")]
async fn main() {
    let mut main_state = MainState::new();

  loop {
      clear_background(BLACK);

      // Update game logic every frame
      for _ in 0..5 {
          main_state.update();
      }

      // Draw game objects
      main_state.draw();

      next_frame().await;
  }
}