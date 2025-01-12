//! ## Some definitions
//! - Logical Pixels (LP) -> our game's "logical" pixel-space as defined in src/constants.rs
//! - Physical Pixels (PP) -> a window manager's representation of the game's window in terms of actual pixels
//!     on the screen, accounting for any dpi scaling
//! - Macroquad Natural Pixels (NP) -> Macroquad's internal "logical" pixel-space which is just `physical pixels / dpi_scale`
//! 
//! We define our viewport in terms of physical pixels (translating from the logical), but otherwise physical pixels have no other use for now..
//! 
//! We use logical pixels for draw calls related to the game's world such as players, npcs, geometry, etc.
//!
//! We use macroquad's natural ("logical") pixels for UI calls, as drawing relatively to the window is common in UI and macroquad already
//! speaks in the window's language this way and handles dpi for us, and we are ofc using their API so... not doing this defeats the point
//! of the engine partially. So really, "Natural" pixels just mean --> "We are talking about the main window now." This is ofc useful for UI.
//!
//! Natural pixel data is useful for draws under [`set_default_camera()`] or any equivalents which encode a window-perfect project matrix in 
//! macroquad's internals (i.e. the viewport == the window manager's physical pixel representation).
pub mod pixel_space;
pub use pixel_space::*;
use crate::prelude::*;

/// All public method calls begin a "lifecycle" for the manager if not already started, which is 
/// essentially an expression for saying the DrawManager has calculated and cached important
/// state for the current frame.
/// 
/// ## Important Requirement
/// You must flush the state and end the lifecycle via [`DrawManager::lifecycle_flush_and_end`]
/// before the end of every frame and after all other method calls.
/// This is important and required because otherwise you will reuse old frame data and may experience strange UI.
#[derive(Default)]
// TODO // Will definitely move around some of the methods/what goes where/names by the next commit
pub struct DrawManager {
    in_lifecycle: bool,
    curr_pset: Option<PSet>,
}

impl DrawManager {
    pub fn new() -> DrawManager {
        DrawManager { ..Default::default() }
    }

    /// Begins a lifecycle if doesn't exist and calculates state , returning it. Otherwise, returns current lifecycle state.
    fn lifecycle_calc_or_get_states(&mut self) -> &PSet  {
        if !self.in_lifecycle {
            self.curr_pset = Some(PSet::current());
            self.in_lifecycle = true;
        }

        // SAFETY: curr_pset must be set by now if in_lifecycle was false. This struct's methods are not exposed publically,
        // so it is safe by lifecycle behavior: in_lifecycle can only be set true by this method. This method can only set it true when
        // in_lifecycle is false, and in_lifecycle inits as false, therefore curr_pset is always in the Option::Some variant
        // by here as no other method controls curr_pset's Some state except for lifecycle_flush_and_end(), which sets in_lifecycle
        // to false before returning.
        unsafe { self.curr_pset.as_ref().unwrap_unchecked() }
    }

    /// Should be drawn with default camera for expected behavior 
    /// ## Side-Effect
    /// Begins a lifecycle if not currently in one.
    pub fn draw_letterboxing_absolute(&mut self) {
        let LetterboxDimensions { top, bottom, left, right } = self.lifecycle_calc_or_get_states().natural.letterbox;
        draw_rectangle(top.x, top.y, top.w, top.h, BLACK);
        draw_rectangle(bottom.x, bottom.y, bottom.w, bottom.h, BLACK);
        draw_rectangle(left.x, left.y, left.w, left.h, BLACK);
        draw_rectangle(right.x, right.y, right.w, right.h, BLACK);
    }

    /// ## Side-Effect
    /// Begins a lifecycle if not currently in one.
    pub fn set_player_camera(&mut self, player_position: Vec2) {
        let pset = self.lifecycle_calc_or_get_states();
        let Viewport { x, y, w, h } = pset.physical.viewport_from_logical;
        let player_camera = Camera2D {
            zoom: pset.logical.camera_zoom,
            target: player_position,
            viewport: Some((x, y, w, h)),
            ..Default::default()
        };
        set_camera(&player_camera);
    }
    
    /// ## Side-Effect
    /// Begins a lifecycle if not currently in one.
    pub fn set_natural_camera(&mut self) {
        let pset = self.lifecycle_calc_or_get_states();
        let Viewport { x, y, w, h } = pset.physical.viewport_from_logical; 
        let (zoom, target) = (pset.natural.camera_zoom, pset.natural.camera_target);
        let natural_camera = Camera2D {
            zoom,
            target,
            viewport: Some((x, y, w, h)),
            ..Default::default()
        };
        set_camera(&natural_camera);
    }
    
    /// ## Side-Effect
    /// Begins a lifecycle if not currently in one.
    pub fn set_ui_camera(&mut self) {
        let pset = self.lifecycle_calc_or_get_states();
        let Viewport { x, y, w, h } = pset.physical.viewport_from_logical; 
        let (zoom, target) = (pset.logical.camera_zoom, pset.logical.camera_target);
        let natural_camera = Camera2D {
            zoom,
            target,
            viewport: Some((x, y, w, h)),
            ..Default::default()
        };
        set_camera(&natural_camera);

    }

    /// ## Important Requirement
    /// Call before frame end and after any other method to end the lifecycle and flush inner state.
    /// This is important and required because otherwise you will reuse old frame data and may experience strange UI.
    ///
    /// Is a no-op if wasn't in a lifecycle.
    pub fn lifecycle_flush_and_end(&mut self) {
        if self.in_lifecycle {
            self.curr_pset = None;
            self.in_lifecycle = false;
        }
    }

    /// Should be drawn with ui camera ([`DrawManager::set_ui_camera`]) for expected behavior.
    /// ## Side-Effect
    /// Begins a lifecycle if not currently in one.
    pub fn draw_dialogue_frame_ui(&mut self) {
        let PxWindow { width: logical_w, height: logical_h } = self.lifecycle_calc_or_get_states().logical.window;
        let d_frame_height = logical_h / 5.0;
        draw_rectangle(0.0, logical_h - d_frame_height, logical_w, d_frame_height, WHITE);
        draw_rectangle_lines(0.0, logical_h - d_frame_height, logical_w, d_frame_height, 3.0, GRAY);
    }

    /// Should be drawn with ui camera ([`DrawManager::set_ui_camera`]) for expected behavior.
    /// ## Side-Effect
    /// Begins a lifecycle if not currently in one.
    pub fn draw_dialogue_text_ui(&mut self, text: &str, maybe_params: Option<TextParams>) {
        let logical_h = self.lifecycle_calc_or_get_states().logical.window.height;
        if let Some(params) = maybe_params {
            dlog!(Level::Info, draw_text_ex(text, 0.0, logical_h - logical_h / 5.0 + params.font_size as f32, params));
        } else {
            // use default ttf font -- we will probably never have a None maybe_params but this is here b/c easy to implement
            draw_text(text, 0.0, logical_h - logical_h / 5.0 + DEFAULT_FONT_SIZE as f32, DEFAULT_FONT_SIZE as f32, DEFAULT_FONT_COLOR);
        }
    }

    
    
}
