pub mod npc;
pub mod bounding;
pub mod update;
pub mod motion;
pub mod draw;
pub mod debug;

pub use npc::*;
pub use bounding::*;
pub use update::*;
pub use motion::*;
pub use draw::*;
pub use debug::*;

use crate::prelude::*;

pub struct Entity {
    pub boundary: Rect,
    pub draw_size: Vec2,
    pub rotation: f32,
    pub texture: Texture2D,
    pub show_hitbox: bool,
    pub velocity: Vec2,
    pub acceleration: Vec2,
}

impl Entity {
    pub async fn build_from_center(
        (x, y): (f32, f32),
        (bwidth, bheight): (f32, f32),
        draw_size: Vec2,
        rotation: f32,
        texture_path: &str, 
    ) -> GResult<Self> {
        let texture = load_texture(texture_path).await?;
    
        let tl_x = x - bwidth / 2.0;
        let tl_y = y - bheight / 2.0;

        Ok(Entity {
            boundary: Rect::new(tl_x, tl_y, bwidth, bheight),
            draw_size,
            rotation,
            texture,
            show_hitbox: false,
            velocity: vec2(0.0, 0.0),
            acceleration: vec2(0.0, 0.0),
        }) 
    }
}

pub trait IsEntity {
    fn ref_entity(&self) -> &Entity;
    fn mut_entity(&mut self) -> &mut Entity;
}

impl IsEntity for Entity {
    fn ref_entity(&self) -> &Entity {
        self
    }
    fn mut_entity(&mut self) -> &mut Entity {
        self
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

impl IsEntity for NPC {
    fn ref_entity(&self) -> &Entity {
        &self.entity
    }
    fn mut_entity(&mut self) -> &mut Entity {
        &mut self.entity
    }
}