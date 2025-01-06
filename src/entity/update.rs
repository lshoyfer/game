use crate::prelude::*;

pub trait Updateable {
    fn update_x(&mut self, dt: f32);
    fn update_y(&mut self, dt: f32);

    fn update(&mut self, dt: f32) {
        self.update_x(dt);
        self.update_y(dt);
    }
}

impl Updateable for Player {
    fn update_x(&mut self, dt: f32) {
        let this = self.mut_entity();
        this.velocity = vec2(0.0, 0.0);
        if is_key_down(KeyCode::A) {
            this.velocity.x -= this.acceleration.x;
        }
        if is_key_down(KeyCode::D) {
            this.velocity.x += this.acceleration.x;
        }
        this.offset(this.velocity * dt);
    }

    fn update_y(&mut self, dt: f32) {
        let this = self.mut_entity();
        this.velocity = vec2(0.0, 0.0);
        if is_key_down(KeyCode::W) {
            this.velocity.y -= this.acceleration.y;
        }
        if is_key_down(KeyCode::S) {
            this.velocity.y += this.acceleration.y;
        }
        this.offset(this.velocity * dt);
    }
}

/* (Player old update function)
    pub fn update(&mut self, dt: f32) {
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
        self.offset(self.velocity * dt);
        self.velocity = vec2(0.0, 0.0);
    }
*/

