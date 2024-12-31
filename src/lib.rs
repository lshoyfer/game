pub mod constants;
pub mod prelude;

use prelude::*;
use macroquad::prelude::*;

pub struct VResTranslationVals {
    pub zoom: Vec2,
    /// Physical Pixels
    pub lb_offsets: Vec2,
    /// Physical Pixels
    pub lb_scales: Vec2,
}

pub fn build_camera(screen_w: f32, screen_h: f32, dpi_scale: f32) -> Camera2D {
    let VResTranslationVals { zoom, lb_offsets, lb_scales } = calculate_camera_and_letterbox_vals(screen_w, screen_h, dpi_scale);
    debug!("Logical Screen Dimensions: vec2({}, {})", screen_w, screen_h);
    debug!("DPI: {}", dpi_scale);
    debug!("Zoom: {:?}", zoom);
    debug!("Letterbox Offsets: {:?}", lb_offsets);
    debug!("Letterbox Scales: {:?}", lb_scales);
    Camera2D {
        zoom,
        target: vec2(VIRTUAL_WIDTH / 2.0, VIRTUAL_HEIGHT / 2.0),
        viewport: Some((
            lb_offsets.x.round() as i32, 
            lb_offsets.y.round() as i32, 
            lb_scales.x.round() as i32, 
            lb_scales.y.round() as i32,
        )),
        ..Default::default()
    }
}

fn calculate_camera_and_letterbox_vals(screen_w: f32, screen_h: f32, dpi_scale: f32) -> VResTranslationVals {
    let lb_scale_x = screen_w / VIRTUAL_WIDTH;
    let lb_scale_y = screen_h / VIRTUAL_HEIGHT;
    let lb_scale = lb_scale_x.min(lb_scale_y);

    let lb_offset_x = (screen_w - VIRTUAL_WIDTH * lb_scale) / 2.0 * dpi_scale;
    let lb_offset_y = (screen_h - VIRTUAL_HEIGHT * lb_scale) / 2.0 * dpi_scale;

    let lb_scaled_w = VIRTUAL_WIDTH * lb_scale * dpi_scale;
    let lb_scaled_h = VIRTUAL_HEIGHT * lb_scale * dpi_scale;

    // Camera has an orthographic rendering zoom API with a clip space of [-1, 1], 
    // such that we must input the translation from our virtual dimensions into 
    // normalized ones as the "zoom" values. So we divide 2 ("2" units of clip space)
    // by each respective dimension for the ratio/translation value. Also, it uses 
    // logical pixels. Don't ask me why.
    let zoom_x = 2.0 / VIRTUAL_WIDTH;
    let zoom_y = 2.0 / VIRTUAL_HEIGHT;

    VResTranslationVals { 
        zoom: vec2(zoom_x, zoom_y), 
        lb_offsets: vec2(lb_offset_x, lb_offset_y), 
        lb_scales: vec2(lb_scaled_w, lb_scaled_h)
    }
}

pub fn draw_basic_testing_pattern() {
    for i_x in (0..(VIRTUAL_WIDTH as i32)).step_by((VIRTUAL_WIDTH / 100.0) as usize) {
        for i_y in (0..(VIRTUAL_HEIGHT as i32)).step_by((VIRTUAL_HEIGHT / 100.0) as usize) {
            // dbg!(i_x, i_y);
            draw_circle(i_x as f32, i_y as f32, 1.0, WHITE);
        }
    }
}

pub struct Player {
    pub position: Vec2,
    pub velocity: Vec2,
    pub speed: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Player {
            position: vec2(x, y),
            velocity: vec2(0.0, 0.0),
            speed: 300.0, // Units per second
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.velocity = vec2(0.0, 0.0);

        if is_key_down(KeyCode::A) {
            self.velocity.x -= self.speed;
        }
        if is_key_down(KeyCode::D) {
            self.velocity.x += self.speed;
        }
        if is_key_down(KeyCode::W) {
            self.velocity.y -= self.speed;
        }
        if is_key_down(KeyCode::S) {
            self.velocity.y += self.speed;
        }

        self.position += self.velocity * dt;

        // Optional: Wrap around the screen
        if self.position.x < 0.0 {
            self.position.x = VIRTUAL_WIDTH;
        } else if self.position.x > VIRTUAL_WIDTH {
            self.position.x = 0.0;
        }

        if self.position.y < 0.0 {
            self.position.y = VIRTUAL_HEIGHT;
        } else if self.position.y > VIRTUAL_HEIGHT {
            self.position.y = 0.0;
        }
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, 25.0, BLUE);
    }
}
