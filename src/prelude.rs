pub use crate::*;
pub use crate::player::*;
pub use crate::entity::*;
pub use crate::constants::*;
pub use crate::macros::*;

pub use macroquad::prelude::*;
pub use macroquad::miniquad::log::Level;

pub type GResult<T> = Result<T, Box<dyn std::error::Error>>;