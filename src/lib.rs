pub mod constants;
pub mod prelude;
pub mod entity;
pub mod macros;
pub mod init;
pub mod ui;

use crate::prelude::*;

pub struct Game {
    pub em: EntityManager,
    pub dm: DialogueManager,
}

impl Game {
    pub fn from_parts(em: EntityManager, dm: DialogueManager) -> Self {
        Game { em, dm }
    }

    pub fn handle_updates_and_collisions(&mut self) {
        let dt = get_frame_time();
        let player = &mut self.em.player;
        let npcs = &mut self.em.npcs;

        player.update_x(dt);
        for npc in npcs.iter_mut() {
            if player.overlaps_excluding_bounds(npc) {
                if player.ref_velocity().x.is_sign_positive() {
                    player.move_by_boundary_to(vec2(npc.ref_boundary().left() - player.bsize().x, player.ref_boundary().y));
                } else {
                    player.move_by_boundary_to(vec2(npc.ref_boundary().right(), player.ref_boundary().y));
                }
                self.dm.load_dialogue(npc);
            }
        }

        player.update_y(dt);
        for npc in npcs.iter_mut() {
            if player.overlaps_excluding_bounds(npc) {
                if player.ref_velocity().y.is_sign_positive() {
                    player.move_by_boundary_to(vec2(player.ref_boundary().x, npc.ref_boundary().top() - player.bsize().y));
                } else {
                    player.move_by_boundary_to(vec2(player.ref_boundary().x, npc.ref_boundary().bottom()));
                }
                self.dm.load_dialogue(npc);
            }
        }
    }

    pub fn handle_ui(&mut self) {
        self.dm.handle_dialogue();
    }
}