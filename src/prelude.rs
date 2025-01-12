pub use crate::*;
pub use crate::entity::*;
pub use crate::constants::*;
pub use crate::init::*;
pub use crate::drawing::*;

pub use macroquad::prelude::*;
pub use log::Level;

pub use std::sync::Arc;

pub type GResult<T> = Result<T, Box<dyn std::error::Error>>;