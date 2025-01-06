pub use crate::*;
pub use crate::entity::*;
pub use crate::constants::*;
pub use crate::macros::*;
pub use crate::init::*;
pub use crate::ui::*;

pub use macroquad::prelude::*;
pub use macroquad::miniquad::log::Level;

pub use std::sync::Arc;

pub type GResult<T> = Result<T, Box<dyn std::error::Error>>;