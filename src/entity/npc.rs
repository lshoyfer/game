pub mod dialogue_manager;
pub use dialogue_manager::*;

use crate::prelude::*;

pub struct NPC {
    pub dialogue: Dialogue,
    pub entity: Entity,
}

impl NPC {
    pub async fn build_from_entity(entity: Entity, dialogue_path: &str) -> GResult<Self> {
        let dialogue = build_dialogue(dialogue_path).await?;
        Ok(NPC { entity, dialogue }) 
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

async fn build_dialogue(dialogue_path: &str) -> GResult<Dialogue> {
    let raw = load_file(dialogue_path).await?;
    let s = String::from_utf8(raw)?;
    Ok(s.lines().map(Box::from).collect())
}