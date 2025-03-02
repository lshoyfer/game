pub mod map_reader;
pub mod custom_usize_option;

pub use map_reader::*;

use crate::prelude::*;
use custom_usize_option::CustomUsizeOption;
use std::io;

#[derive(Debug)]
pub struct Line {
    pub inner: Vec4,
}


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
/// Helper struct to identify and lazy-load all textures
pub struct TextureIndex {
    space_index: NonZeroUsize, // Allows for this struct to be NPO'd, and "0" is designated as the player's space index (we use a function exclusively for player texture loading to get around this)
    texture_index: usize,
    inner_index: CustomUsizeOption,
}

impl TextureIndex {
    fn new(space_index: NonZeroUsize, texture_index: usize, maybe_inner_index: CustomUsizeOption) -> Self {
        TextureIndex { space_index, texture_index, inner_index: maybe_inner_index }
    }

    pub async fn load_texture(&self) -> GResult<(Texture2D, Option<DrawTextureParams>)> {
        let mut path = String::from(ROOT_TEXTURES_PATH);

        let (space_dir, (texture_type, texture_file_name)) = match self.space_index.get() {
           1 => ("npc", NPC_TEXTURES[self.texture_index]),
           2 => ("geometry", GEOMETRY_TEXTURES[self.texture_index]),
           _ => return Err(io::Error::new(io::ErrorKind::AddrNotAvailable, "Requested texture's space_index does not have a corresponding directory").into())
        };

        path.push('/');
        path.push_str(space_dir);
        path.push('/');
        path.push_str(texture_file_name);

        let texture = load_texture(&path).await?;

        let params = match texture_type {
            TextureType::Atlas(offset) => {
                if self.inner_index.is_none() {
                    dlog!(Level::Error, "Tried to load texture with this index [{} {:?} {:?}]. TextureIndex's inner_index is None despite texture_type being an Atlas. Some map code does not match the textures static arrays somewhere. Panicking.", self.space_index, self.texture_index, self.inner_index);
                    panic!("Was going to unwrap a None value, check Error log.");
                }
                // SAFETY: Hard to really uphold this invariant as it depends on my cross checking hence we emit an error and panic in case I fucked up,
                // but this should always be Some() if the texture type is an Atlas. I doubt I will but if I make a level editor this would be an easier
                // invariant to uphold.
                let inner_index = unsafe { self.inner_index.unwrap_unchecked() };
                Some(DrawTextureParams {
                    source: Some(Rect::new((offset * inner_index) as f32, 0.0, offset as f32, texture.height())),
                    ..Default::default()
                })
            }
            TextureType::Standalone => None,
        };

        Ok((texture, params))
    }

    pub async fn load_player_texture(&self) -> GResult<(Texture2D, Option<DrawTextureParams>)> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Geometry {
    pub kind: GeometryType,
    pub t_index: Option<TextureIndex>,
}

impl Geometry {
    pub fn new_rect(x: f32, y: f32, w: f32, h: f32, t_index: Option<TextureIndex>) -> Self {
        Geometry { kind: GeometryType::Rect(Rect::new(x, y, w, h)), t_index }
    }
}

#[derive(Debug)]
pub enum GeometryType {
    Rect(Rect)
    // is_wholly_inside_a_chunk: bool,
}

// impl RectBounded for Geometry {
//     fn ref_boundary(&self) -> &Rect {
//         &self.boundary
//     }

//     fn mut_boundary(&mut self) -> &mut Rect {
//         &mut self.boundary
//     }
// }

/* // REVIEW
/// 256 x 256 logical pixels, contains a list of geometry that are either wholly or partially inside of it
pub struct Chunk {
    pub inner: Vec<Geometry>,
}

pub struct GeometryMap {
    pub chunks: Vec<Chunk>
}
*/

#[derive(Debug)]
pub struct GeometryMap {
    pub inner: Vec<Geometry>
}

// TODO //
/*
(Remember that in the parser the texture inputs go at the end instead of how it is currently in the test_map file)
- CustomUsizeOption might be useless 
    - Unused None, we de-coupled the texture map from the texture inputs, not sure what's the right design decision
    - Maybe make a texture file reader v.s. the current geometry map one? I dont think so.
- Probably rename inner_index to atlas_offset or offset_index or offset
- Implement load_player_texture
- Check if your changes even fucking work
transcribe this russian into plaintext (using russian characters ofc)
*/