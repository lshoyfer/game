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

async fn build_dialogue(dialogue_path: &str) -> GResult<Dialogue> {
    let raw = load_file(dialogue_path).await?;
    let s = String::from_utf8(raw)?;
    Ok(s.lines().map(Box::from).collect())
}