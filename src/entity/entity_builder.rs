use crate::prelude::*;

pub struct EntityBuilder {
    eid_count: usize
}

impl EntityBuilder {
    pub fn new() -> Self {
        EntityBuilder { eid_count: 0 }
    }

    pub async fn build_entity_from_center(
        &mut self,
        (x, y): (f32, f32),
        (bwidth, bheight): (f32, f32),
        draw_size: Vec2,
        rotation: f32,
        texture_path: &str,
    ) -> GResult<Entity> {
        let e = Entity::build_from_center(
            (x, y),
            (bwidth, bheight),
            draw_size,
            rotation,
            texture_path,
            self.eid_count,
        );
        self.eid_count += 1;
        e.await
    }

    pub async fn build_entity_from_boundary(
        &mut self,
        boundary: Rect,
        draw_size: Vec2,
        rotation: f32,
        texture_path: &str,
    ) -> GResult<Entity> {
        let e = Entity::build_from_boundary(
            boundary,
            draw_size,
            rotation,
            texture_path,
            self.eid_count,
        );
        self.eid_count += 1;
        e.await
    }

    pub async fn init_player(&mut self) -> GResult<Player> {
        let p_entity = self.build_entity_from_center(
            (0.0, 0.0),
            (PLAYER_SIZE.x, PLAYER_SIZE.y),
            PLAYER_SIZE,
            0.0, 
            PLAYER_SPRITE_PATH
        ).await?;
        Ok(Player::from_entity(p_entity))
    }
    
    pub async fn init_npcs(&mut self) -> GResult<Vec<NPC>> {
        let npc1_entity = self.build_entity_from_center(
            (300.0, 360.0),
            (70.0, 70.0), 
            vec2(120.0, 120.0),
            0.0,
            "assets/sprites/test_npc.png",
            
        ).await?;
        let npc1 = NPC::build_from_entity(
            npc1_entity,
            "assets/dialogue/test_dialogue.txt"
        ).await?;

        Ok(vec![npc1])
    }
}

