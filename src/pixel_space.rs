//! ## Some definitions
//! - Logical Pixels (LP) -> our game's "logical" pixel-space as defined in src/constants.rs
//! - Physical Pixels (PP) -> a window manager's representation of the game's window in terms of actual pixels
//!     on the screen, accounting for any dpi scaling
//! - Macroquad Natural Pixels (NP) -> Macroquad's internal "logical" pixel-space which is just `physical pixels / dpi_scale`
//! 
//! We use logical pixels for most draw calls in the game including UI.
//! We define a viewport in terms of physical pixels, translating from either the logical or natural pixel spaces. 
//! We use our viewport from the logical to draw UI elements relatively, keeping letterboxing automatically in mind.
//! We use both the physical window and the physical viewport translation of logical pixel space to build the letterboxing,
//! which is then currently drawn in natural pixel space. 
//! 
//! //REVIEW// I should reexamine some of the specifics here, as I might not generally need natural pixels, 
//! for they could just be an arbitrary holdout from old code I let slip in that is bloat. Or... it's useful
//! for briding low and high dpi... Probably actually. Will have to see, but right now this is not important. 
//!
//! Natural pixel data is useful for draws under [`set_default_camera()`] or any equivalents which encode a window-perfect project matrix in 
//! macroquad's internals (i.e. the viewport == the window manager's physical pixel representation).
pub mod logical;
pub mod physical;
pub mod natural;

pub use logical::*;
pub use physical::*;
pub use natural::*;

use crate::prelude::*;

/// General struct; Holds pixel data in terms of any pixel view of the window
pub struct PxWindow {
    pub width: f32,
    pub height: f32,
}

/// Holds set of all pixel space data used for camera management and drawing
pub struct PSet {
    pub logical: LPData,
    pub physical: PPData,
    pub natural: NPData,
}

impl PSet {
    /// Queries and calculates all pertinent window/pixel data for all pixel spaces for the current frame
    pub fn current() -> Self {
        let dpi_scale = screen_dpi_scale();

        let p_window = PxWindow::query_physical_window();
        let l_window = PxWindow { width: LOGICAL_WIDTH, height: LOGICAL_HEIGHT };
        let n_window = PxWindow { width: p_window.width / dpi_scale, height: p_window.height / dpi_scale };

        // Camera has an orthographic rendering zoom API with a clip space of [-1, 1], 
        // such that we must input the translation from our logical dimensions into 
        // normalized ones as the "zoom" values. So we divide 2 ("2 units" of clip space)
        // by each respective dimension for the ratio/translation value. It consequently
        // provides the map for our natural/logical pixels to their physical viewports.
        let natural_zoom = vec2(2.0 / n_window.width, 2.0 / n_window.height);
        let logical_zoom = vec2(2.0 / l_window.width, 2.0 / l_window.height);

        let natural_camera_target = vec2(n_window.width / 2.0, n_window.height / 2.0);
        let logical_camera_target = vec2(l_window.width / 2.0, l_window.height / 2.0);

        let viewport_from_logical = PSet::calc_viewport_from_logical(&l_window, &p_window);
        let viewport_from_natural = Viewport { x: 0, y: 0, w: p_window.width as i32, h: p_window.height as i32 };

        let letterbox = PSet::calculate_letterbox_dimensions(&n_window, &viewport_from_logical, dpi_scale);

        PSet {
            logical: LPData { window: l_window, camera_zoom: logical_zoom, camera_target: logical_camera_target },
            physical: PPData { viewport_from_logical, viewport_from_natural, window: p_window },
            natural: NPData { window: n_window, camera_zoom: natural_zoom, camera_target: natural_camera_target, letterbox },
        }
    }

    fn calc_viewport_from_logical(l_window: &PxWindow, p_window: &PxWindow) -> Viewport {
        let PxWindow { width: logical_w, height: logical_h } = l_window;
        let PxWindow { width: screen_w, height: screen_h } = p_window;

        let vp_scale_x = screen_w / logical_w;
        let vp_scale_y = screen_h / logical_h;
        let vp_scale = vp_scale_x.min(vp_scale_y);

        let vp_x = (screen_w - logical_w * vp_scale) / 2.0;
        let vp_y = (screen_h - logical_h * vp_scale) / 2.0;

        let vp_w = logical_w * vp_scale;
        let vp_h = logical_h * vp_scale;

        Viewport { 
            x: vp_x.round() as i32,
            y: vp_y.round() as i32,
            w: vp_w.round() as i32,
            h: vp_h.round() as i32,
        }
    }

    fn calculate_letterbox_dimensions(n_window: &PxWindow, viewport_from_logical: &Viewport, dpi_scale: f32) -> LetterboxDimensions {
        let (vp_y, vp_h) = (
            viewport_from_logical.y as f32 / dpi_scale,
            viewport_from_logical.h as f32 / dpi_scale
        );
        let top = Rect::new(0.0, 0.0, n_window.width, vp_y);
        let bottom = Rect::new(0.0, vp_y + vp_h, n_window.width, vp_y);

        let (vp_x, vp_w) = (
            viewport_from_logical.x as f32 / dpi_scale,
            viewport_from_logical.w as f32 / dpi_scale
        );
        let left = Rect::new(0.0, 0.0, vp_x, n_window.height);
        let right = Rect::new(vp_x + vp_w, 0.0, vp_x, n_window.height);

        LetterboxDimensions { top, bottom, left, right }
    }

}