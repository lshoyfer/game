use crate::prelude::*;

pub struct NPC {
    pub dialogue: String,
    pub entity: Entity
}

impl NPC {
    pub async fn build_from_entity(
        entity: Entity,
        dialogue_path: &str
    ) -> GResult<Self> {
        let dialogue = load_file(dialogue_path).await?;
        let dialogue = String::from_utf8(dialogue)?;

        Ok(NPC { entity, dialogue }) 
    }

    pub async fn build_from_center(
        (x, y): (f32, f32),
        (bwidth, bheight): (f32, f32),
        draw_size: Vec2,
        rotation: f32,
        texture_path: &str, 
        dialogue_path: &str,
    ) -> GResult<Self> {
        let entity = Entity::build_from_center(
            (x, y), 
            (bwidth, bheight),
            draw_size, 
            rotation, 
            texture_path
        ).await?;
        let dialogue = load_file(dialogue_path).await?;
        let dialogue = String::from_utf8(dialogue)?;
    
        Ok(NPC {
            entity,
            dialogue,
        }) 
    }
}