use crate::prelude::*;

/// Handles natural pixels
pub struct LetterboxDimensions {
    pub top: Rect,
    pub bottom: Rect,
    pub left: Rect,
    pub right: Rect
}

/// Holds data relevant for the natural pixel space
pub struct NPData {
    /// Holds data relevant to the natural pixel view of the window
    pub window: PxWindow,
    pub camera_zoom: Vec2,
    /// Center of natural pixel space
    pub camera_target: Vec2,
    pub letterbox: LetterboxDimensions,
}