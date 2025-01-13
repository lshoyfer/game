use crate::prelude::*;

/// Holds data relevant to the logical pixel space
pub struct LPData {
    /// Holds data relevant to the logical pixel view of the window.
    pub window: PxWindow,
    /// Mapping from our logical pixels to physical viewport
    pub camera_zoom: Vec2,
    /// Center of logical pixel space (usually is the player's position, but if we are drawing UI it becomes this)
    pub camera_target: Vec2,
}