use crate::prelude::*;

/// Generic physical pixel representation of any pixel space
pub struct Viewport {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl PxWindow {
    /// Queries current frame's window state in terms of physical pixels
    pub fn query_physical_window() -> Self {
        let (width, height) = miniquad::window::screen_size(); // returns physical pixels
        if height <= 0.0 || width <= 0.0 || width.is_infinite() || width.is_nan() || height.is_infinite() || height.is_nan() {
            // FIXME // consider height and/or width == 0 edge cases, which is improbable but it's good practice for later
            // FIXME // would a 0x0 html canvas report 0x0 on these functions?
            // Currently I believe there's no need to handle these edge cases as the engine itself has no regard for these edge cases
            // internally as far as I can tell. The only example, on Camera3D's matrix implementation for the Camera trait, has a possible
            // branch where it computes aspect ratio via (width / height) and the NaN/etc cases do not seem to be handled anywhere down the
            // line as far as I can tell. Even though we are not currently using Camera3D, it does appear to be a real supported feature of
            // the engine. Therefore I will trust it *for now* but test definitively later. Also my IDE is not showing me any actual uses
            // of the Camera3d matrix function where that aspect ratio is calculated, so....... who really knows.
            dlog!(Level::Error, width, height);
        }
        PxWindow { width, height }
    }
}


/// Holds data relevant to the physical pixel space
pub struct PPData {
    /// Physical pixel representation of our logical pixel space
    pub viewport_from_logical: Viewport,
    /// Physical pixel representation of the natural pixel space
    pub viewport_from_natural: Viewport,
    /// Holds data relevant to the physical pixel view of the window.
    pub window: PxWindow,
}

