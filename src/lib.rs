pub mod constants;
pub mod prelude;
pub mod entity;
pub mod macros;
pub mod init;
pub mod drawing;

use crate::prelude::*;

pub struct Game {
    pub em: EntityManager,
    pub dm: DialogueManager,
    pub um: DrawManager,
}

impl Game {
    pub fn from_parts(em: EntityManager, dm: DialogueManager, um: DrawManager) -> Self {
        Game { em, dm, um }
    }

    pub fn init_view_and_update_entites(&mut self) {
        self.um.draw_letterboxing_absolute();
        self.handle_entity_updates_and_collisions();
        self.um.set_player_camera(self.em.ref_player().position());
        draw_rectangle_lines(0.0, 0.0, LOGICAL_WIDTH, LOGICAL_HEIGHT, 1.0, BLACK);
    }

    pub fn draw_loaded_entites(&self) {
        self.em.draw_loaded();
    }
    
    fn handle_entity_updates_and_collisions(&mut self) {
        let dt = get_frame_time();
        let player = &mut self.em.player;
        let npcs = &mut self.em.npcs;

        player.update_x(dt);
        for npc in npcs.iter_mut() {
            if player.overlaps_excluding_bounds(npc) {
                if player.ref_velocity().x.is_sign_positive() {
                    player.move_by_origin_to(vec2(npc.ref_boundary().left() - player.bsize().x, player.ref_boundary().y));
                } else {
                    player.move_by_origin_to(vec2(npc.ref_boundary().right(), player.ref_boundary().y));
                }
                self.dm.load_dialogue(npc);
            }
        }

        player.update_y(dt);
        for npc in npcs.iter_mut() {
            if player.overlaps_excluding_bounds(npc) {
                if player.ref_velocity().y.is_sign_positive() {
                    player.move_by_origin_to(vec2(player.ref_boundary().x, npc.ref_boundary().top() - player.bsize().y));
                } else {
                    player.move_by_origin_to(vec2(player.ref_boundary().x, npc.ref_boundary().bottom()));
                }
                self.dm.load_dialogue(npc);
            }
        }
    }

    pub fn handle_ui(&mut self) {
        self.um.set_ui_camera();

        if let Some((dpart, maybe_params)) = self.dm.handle_dialogue() {
            self.um.draw_dialogue_frame_ui();
            draw_rectangle(0.,0.,100.,100.,RED);
            self.um.draw_dialogue_text_ui(dpart, maybe_params);
        }
    }

    pub fn clean_up(&mut self) {
        self.um.lifecycle_flush_and_end();
    }
}