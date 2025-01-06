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