use crate::prelude::*;

pub struct Player {
    pub entity: Entity,
}

impl Player {
    pub fn from_entity(mut entity: Entity) -> Self {
        entity.velocity = vec2(0.0, 0.0);
        entity.acceleration = vec2(400.0, 400.0);
        Player { entity }
    }
}

impl IsEntity for Player {
    fn ref_entity(&self) -> &Entity {
        &self.entity
    }
    fn mut_entity(&mut self) -> &mut Entity {
        &mut self.entity
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