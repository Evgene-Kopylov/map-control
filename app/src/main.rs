use macroquad::prelude::*;
mod settings;
use settings::{
    GROUND_COLOR,
    LINE_COLOR,
};
use crate::settings::{TRANSPARENT};


struct Camera {
    x: f32,
    y: f32,
    speed: f32,
    step: f32,
    zoom: f32,
    prev_mouse_position: (f32, f32),
}

impl Camera {
    fn start() -> Camera {
        Camera {
            x: 0.,
            y: 0.,
            speed: 500.,
            step: 100.,
            zoom: 1.,
            prev_mouse_position: (0., 0.),
        }
    }

    fn draw_coordination_greed(&self) {
        let range_x = ((screen_width() + self.x.abs()) / (self.step * self.zoom)) as i32;
        for i in -range_x..=range_x {
            let x = (i as f32) * (self.step * self.zoom) + self.x;
            if x > 0. && x < screen_width() {
                draw_line(
                    x, 0.,
                    x, screen_height(),
                    1.,
                    LINE_COLOR);
            }
        }

        let range_y = ((screen_height() + self.y.abs()) / (self.step * self.zoom)) as i32;
        for i in -range_y..=range_y {
            let y = (i as f32) * (self.step * self.zoom) + self.y;
            if y > 0. && y < screen_height() {
                draw_line(
                    0.,y,
                    screen_width(),y,
                    1.,
                    LINE_COLOR);
            }
        }
    }

    fn draw_hexagon(&self) {
        let initial_position: Vec2 = Vec2::new(200., 300.);
        let pos: Vec2 = Vec2::new(
            initial_position.x,
            initial_position.y
        );
        draw_hexagon(
            pos.x * self.zoom + self.x,
            pos.y * self.zoom + self.y,
            self.step * self.zoom,
            1.,
            true,
            DARKGRAY,
            TRANSPARENT
        )
    }

    fn update(&mut self, dt: f32) {
        if is_mouse_button_down(MouseButton::Middle) {
            self.x += mouse_position().0 - self.prev_mouse_position.0;
            self.y += mouse_position().1 - self.prev_mouse_position.1;
        }
        self.prev_mouse_position = mouse_position();

        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.x -= dt * self.speed;
        }

        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.x += dt * self.speed;
        }

        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            self.y -= dt * self.speed;
        }

        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            self.y += dt * self.speed;
        }

        let mw = mouse_wheel().1;
        if mw != 0. {
            let dmw = mw * 0.01 * 0.01 * self.speed;

            if self.zoom >= 0.1 || dmw > 0. {
                let x = mouse_position().0 - self.x;
                let dx = x * ((self.step * self.zoom) + dmw) / (self.step * self.zoom) - x;
                self.x -= dx;

                let y = mouse_position().1 - self.y;
                let dy = y * ((self.step * self.zoom) + dmw) / (self.step * self.zoom) - y;
                self.y -= dy;

                self.zoom += dmw / 100.;
            }
        }
    }
}



#[macroquad::main("breakout")]
async fn main() {
    let mut line = Camera::start();

    loop {
        clear_background(GROUND_COLOR);

        line.update(get_frame_time());
        line.draw_coordination_greed();
        line.draw_hexagon();

        next_frame().await
    }
}
