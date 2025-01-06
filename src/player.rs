use crate::prelude::*;

pub struct Player {
    pub entity: Entity,
}

impl Player {
    /// Initializes a player with the top-left
    /// of their bounding box at (0, 0)
    // pub async fn new() -> Self {
    //     let texture = load_texture(PLAYER_SPRITE_PATH)
    //         .await
    //         .expect("Player asset should exist in assets folder");
    //     Player {
    //         boundary: Rect::new(0.0, 0.0, PLAYER_SIZE.x, PLAYER_SIZE.y),
    //         velocity: vec2(0.0, 0.0),
    //         speed: 400.0, // per second
    //         texture,
    //     }
    // }

    pub async fn build_from_center(
        (x, y): (f32, f32),
        (bwidth, bheight): (f32, f32),
        draw_size: Vec2,
        rotation: f32,
        texture_path: &str, 
    ) -> GResult<Self> {
        let mut entity = Entity::build_from_center(
            (x, y), 
            (bwidth, bheight),
            draw_size, 
            rotation, 
            texture_path
        ).await?;
        entity.velocity = vec2(0.0, 0.0);
        entity.acceleration = vec2(400.0, 400.0);
        Ok(Player { entity })
    }
}