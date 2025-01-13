// Crate Globs
pub use crate::entity::*;
pub use crate::constants::*;
pub use crate::init::*;
pub use crate::pixel_space::*;

// Crate Modules
pub use crate::window_drawing;
pub use crate::camera;

// Crate Items
pub use crate::Game;
pub use crate::dlog;

// Dependencies
pub use macroquad::prelude::*;
pub use log::Level;

// STD Items
pub use std::sync::Arc;
pub use std::mem::MaybeUninit;

// Other
pub type GResult<T> = Result<T, Box<dyn std::error::Error>>;