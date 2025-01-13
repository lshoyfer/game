pub mod player;
pub mod npc;
pub mod entity_builder;
pub mod entity_manager;

pub use player::*;
pub use npc::*;
pub use entity_builder::*;
pub use entity_manager::*;

use crate::prelude::*;

pub struct Entity {
    pub boundary: Rect,
    pub draw_size: Vec2,
    pub rotation: f32,
    pub texture: Texture2D,
    pub show_hitbox: bool,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub id: usize,
}

impl Entity {
    async fn build_from_center(
        (x, y): (f32, f32),
        (bwidth, bheight): (f32, f32),
        draw_size: Vec2,
        rotation: f32,
        texture_path: &str,
        id: usize,
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
            id
        }) 
    }

    async fn build_from_boundary(
        boundary: Rect,
        draw_size: Vec2,
        rotation: f32,
        texture_path: &str,
        id: usize
    ) -> GResult<Self> {
        let texture = load_texture(texture_path).await?;
    
        Ok(Entity {
            boundary,
            draw_size,
            rotation,
            texture,
            show_hitbox: false,
            velocity: vec2(0.0, 0.0),
            acceleration: vec2(0.0, 0.0),
            id
        }) 
    }
}

impl IsEntity for Entity {
    fn ref_entity(&self) -> &Entity {
        self
    }
    fn mut_entity(&mut self) -> &mut Entity {
        self
    }
}

pub trait IsEntity {
    fn ref_entity(&self) -> &Entity;
    fn mut_entity(&mut self) -> &mut Entity;

    fn id(&self) -> usize {
        self.ref_entity().id
    }
}

impl<T: IsEntity> RectBounded for T {
    fn ref_boundary(&self) -> &Rect {
        &self.ref_entity().boundary
    }
    fn mut_boundary(&mut self) -> &mut Rect {
        &mut self.mut_entity().boundary
    }
}

impl<T: IsEntity> Drawable for T {
    fn draw(&self) {
        let this = self.ref_entity();
        let params = DrawTextureParams {
            dest_size: Some(this.draw_size),
            rotation: this.rotation,
            ..Default::default()
        };
        let tl = this.boundary.center() - this.draw_size / 2.0; 
        draw_texture_ex(&this.texture, tl.x, tl.y, WHITE, params);

        if this.show_hitbox {
            let Rect { x, y, w, h } = this.boundary;
            draw_rectangle_lines(x, y, w, h, 4.0, BLACK);
        }
    }
}

impl<T: IsEntity> Teleportable for T {
    fn move_by_origin_to(&mut self, destination: Vec2) {
        self.mut_entity().boundary.move_to(destination);
    }

    fn move_by_center_to(&mut self, destination: Vec2) {
        let boundary = &mut self.mut_entity().boundary;
        boundary.move_to(destination - boundary.size() / 2.0);
    }

    fn offset(&mut self, offset: Vec2) {
       self.mut_boundary().x += offset.x;
       self.mut_boundary().y += offset.y;
    }
}

impl<T: IsEntity> Moveable for T {
    fn velocity(&self) -> Vec2 {
        self.ref_entity().velocity
    }

    fn ref_velocity(&self) -> &Vec2 {
        &self.ref_entity().velocity
    }
    
    fn mut_velocity(&mut self) -> &mut Vec2 {
        &mut self.mut_entity().velocity
    }
}