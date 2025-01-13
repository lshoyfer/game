// Crate Globs
pub use crate::entity::*;
pub use crate::constants::*;
pub use crate::init::*;
pub use crate::pixel_space::*;
pub use crate::geometry::*;
pub use crate::traits::*;

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
pub use std::error::Error;
pub use std::fmt::{ Debug, Display };

// Other
pub type GResult<T> = Result<T, Box<dyn Error>>;