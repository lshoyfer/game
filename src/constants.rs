//! Contains constants, static items, basic global type aliases, and definitions related to all thereof.
use crate::prelude::*;
use TextureType::*;

pub const LOGICAL_WIDTH: f32 = 1280.0;
pub const LOGICAL_HEIGHT: f32 = 720.0;

pub const PLAYER_SIZE: Vec2 = vec2(128.0, 128.0);
pub const PLAYER_SPRITE_PATH: &str = "assets/sprites/test_sprite2.png";

pub const DEFAULT_FONT_TTF_PATH: &str = "./assets/fonts/IM_Fell_English/IMFellEnglish-Regular.ttf";
pub const DEFAULT_FONT_SIZE: u16 = 50;
pub const DEFAULT_FONT_COLOR: Color = BLACK;

pub const ROOT_TEXTURES_PATH: &str = "./assets/textures";

pub type GResult<T> = Result<T, Box<dyn Error>>;

#[derive(Clone, Copy)]
pub enum TextureType {
    Standalone,
    /// usize gives the offset per atlas index
    Atlas(usize)
}

// Works for now, may work forever :^)
pub static NPC_TEXTURES: &[(TextureType, &str)] = &[
    (Standalone, "test_npc.png")
];

pub static GEOMETRY_TEXTURES: &[(TextureType, &str)] = &[
    (Standalone, "LongSongNoHeart.png"),

];

pub const PLAYER_TEXTURE_ATLAS: &str = "test_player_sprite";