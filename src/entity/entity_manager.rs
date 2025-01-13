use crate::prelude::*;

pub struct EntityManager {
    pub player: Player,
    pub npcs: Vec<NPC>,
}

impl EntityManager {
    /// Draws all loaded/enabled entities
    pub fn draw_loaded(&self) {
        self.npcs.iter().for_each(NPC::draw);
        self.player.draw();
    }

    pub fn ref_player(&self) -> &Player {
        &self.player
    }

    pub fn mut_player(&mut self) -> &mut Player {
        &mut self.player
    }

    pub fn ref_npcs(&self) -> &[NPC] {
        &self.npcs
    }

    pub fn mut_npcs(&mut self) -> &mut [NPC] {
        &mut self.npcs
    }
}