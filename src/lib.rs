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
    /// Current loaded map
    pub map: GeometryMap,
    /// Current loaded geometry textures
    pub geometry_textures: HashMap<TextureIndex, (Texture2D, Option<DrawTextureParams>)>,
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
            window_drawing::draw_dialogue_text(pset, dpart, maybe_params);
        }
    }
}

impl Game {
    pub fn init_player_view_and_update_entites(&mut self) {
        self.handle_entity_updates_and_collisions();
        camera::set_player_camera(&self.pset, self.em.ref_player().position());
        // draw_rectangle_lines(0.0, 0.0, LOGICAL_WIDTH, LOGICAL_HEIGHT, 1.0, BLACK);
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

        // for geometry in self.map.

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

impl Game {
    /// Ensure in player camera [`camera::set_player_camera`] for proper behavior.
    pub async fn draw_map(&mut self) -> GResult<()> {
        for geometry in self.map.inner.iter() {
            if let Some(t_index) = geometry.t_index.as_ref() {
                match geometry.kind {
                    GeometryType::Rect(Rect { x, y, .. }) => {
                        // I don't think entry API here is workable, even with an out pointer for the error, as we need to pass an async function,
                        // so this pattern is required instead -- It's more sensible for our case with the Err anyways.
                        let (texture, params) = if self.geometry_textures.contains_key(&t_index) {
                            &self.geometry_textures[&t_index]
                        } else {
                            let _ = self.geometry_textures.insert(*t_index, t_index.load_texture().await?);
                            &self.geometry_textures[&t_index]
                        };

                        if let Some(params) = params {
                            draw_texture_ex(texture, x, y, BLACK, params.clone());
                        } else {
                            draw_texture(texture, x, y, BLACK);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}