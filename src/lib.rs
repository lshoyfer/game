pub mod constants;
pub mod prelude;
pub mod entity;
pub mod macros;
pub mod init;
pub mod pixel_space;
pub mod camera;
pub mod window_drawing;
pub mod geometry;
pub mod traits;

use crate::prelude::*;

pub struct Game {
    pub eb: EntityBuilder,
    pub em: EntityManager,
    pub dm: DialogueManager,
    /// Current frame's pixel space state
    pub pset: PSet,
}

impl Game {
    // Will do an initial unnecessary recalculation for the first frame
    // but avoids constant Option/MaybeUninit shenangians for every frame
    // after if we want pset inside the Game struct
    pub fn start_frame(&mut self) {
        self.pset = PSet::current();
        camera::set_natural_camera(&self.pset);
        clear_background(WHITE);
        window_drawing::draw_letterboxing_natural(&self.pset);
    }

    pub async fn next_frame(&mut self) {
        next_frame().await
    }
}

impl Game {
    pub fn handle_ui(&mut self) {
        let pset = &self.pset;
        camera::set_ui_camera(pset);
        
        if let Some((dpart, maybe_params)) = self.dm.handle_dialogue() {
            window_drawing::draw_dialogue_frame(pset);
            draw_rectangle(0.,0.,100.,100.,RED);
            window_drawing::draw_dialogue_text(pset, dpart, maybe_params);
        }
    }
}

impl Game {
    pub fn init_player_view_and_update_entites(&mut self) {
        self.handle_entity_updates_and_collisions();
        camera::set_player_camera(&self.pset, self.em.ref_player().position());
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
}