pub mod player;
pub mod npc;
pub mod bounding;
pub mod update;
pub mod motion;
pub mod draw;
pub mod debug;

pub use player::*;
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

pub trait IsEntity {
    fn ref_entity(&self) -> &Entity;
    fn mut_entity(&mut self) -> &mut Entity;

    fn id(&self) -> usize {
        self.ref_entity().id
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

pub struct EntityManager {
    pub player: Player,
    pub npcs: Vec<NPC>,
}

impl EntityManager {
    pub fn from_parts(player: Player, npcs: Vec<NPC> ) -> Self {
        EntityManager { player, npcs }
    }

    
    /// Draws all loaded/enabled entities
    pub fn draw_loaded(&self) {
        self.npcs.iter().for_each(NPC::draw);
        self.player.draw();
    }

    pub fn ref_player(&mut self) -> &Player {
        &self.player
    }

    pub fn mut_player(&mut self) -> &mut Player {
        &mut self.player
    }

    pub fn ref_npcs(&mut self) -> &[NPC] {
        &self.npcs
    }

    pub fn mut_npcs(&mut self) -> &mut [NPC] {
        &mut self.npcs
    }
}