pub mod player;
pub mod npc;
pub mod bounding;
pub mod update;
pub mod motion;
pub mod draw;
pub mod debug;

pub use player::*;
pub use npc::*;
pub use bounding::*;
pub use update::*;
pub use motion::*;
pub use draw::*;
pub use debug::*;

use crate::prelude::*;

pub struct Entity {
    pub boundary: Rect,
    pub draw_size: Vec2,
    pub rotation: f32,
    pub texture: Texture2D,
    pub show_hitbox: bool,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub id: usize,
}

impl Entity {
    async fn build_from_center(
        (x, y): (f32, f32),
        (bwidth, bheight): (f32, f32),
        draw_size: Vec2,
        rotation: f32,
        texture_path: &str,
        id: usize,
    ) -> GResult<Self> {
        let texture = load_texture(texture_path).await?;
    
        let tl_x = x - bwidth / 2.0;
        let tl_y = y - bheight / 2.0;

        Ok(Entity {
            boundary: Rect::new(tl_x, tl_y, bwidth, bheight),
            draw_size,
            rotation,
            texture,
            show_hitbox: false,
            velocity: vec2(0.0, 0.0),
            acceleration: vec2(0.0, 0.0),
            id
        }) 
    }

    async fn build_from_boundary(
        boundary: Rect,
        draw_size: Vec2,
        rotation: f32,
        texture_path: &str,
        id: usize
    ) -> GResult<Self> {
        let texture = load_texture(texture_path).await?;
    
        Ok(Entity {
            boundary,
            draw_size,
            rotation,
            texture,
            show_hitbox: false,
            velocity: vec2(0.0, 0.0),
            acceleration: vec2(0.0, 0.0),
            id
        }) 
    }
}

// Although this is a bit of an OOP-method-hell, it is quite convenient.
// I'll keep the following comment here for a few commits for reference and remove it later.
// 
// I could also have forced a `DerefMut<Target=Entity>`-style but that incurs 
// 1. Much greater boilerplate for equivalent usage
//     - For example, we can't just implement all the other traits under
//     for `Entity`, we must also implement for `T: DerefMut<Target=Entity>`,
//     as otherwise forcing deref coercion for those methods becomes a 
//     burden from the borrow checking, for instance, the underneath bulleted code would
//     no longer compile if we did not also double up every implementation for
//     types with `DerefMut<Target=Entity>` due to a full mutable borrow on the
//     first mention of the `player` place (i.e. that `player` becomes `(&mut *player)`)
//     which persists for that expression, meaning the immutable `player.bsize()` now
//     becomes a borrow check violation, as does the `player.ref_boundary()` method call.
//     By adding the method implementation to the `Player` type itself (via a blanket 
//     `T: DerefMut<Target=Entity>`) as opposed to requring deref to `Entity`, we absolve
//     this issue as now it is indentical in form/behavior to our current `IsEntity` blanket
//     impls. It does so because now `.move_by_origin_to` is a direct method call that waits
//     to mutably borrow the receiver (`&mut self`) until after we fully compute the arguments,
//     as opposed to requiring a mutable borrow for deref coercion just to get the correct method
//     before even getting to the parameters. Or at least, that's roughly how I think it works.
//         - `player.move_by_origin_to(vec2(npc.ref_boundary().left() - player.bsize().x, player.ref_boundary().y));`
// 2. Antipatterns and semantic differences in meaning
//    - Doing the aforementioned `DerefMut` blankets sort of go against the spirit of `Deref`/`DerefMut`
//      as you are duplicating most/all methods of `Entity` to the `DerefMut<Target=Entity>` types
//      directly, so the point of `DerefMut` boils down to having direct field access rather than
//      having to type `.entity` or `.(ref|mut)_entity()` as a precursor... It's nice I guess, but
//      strange to bring in such a heavy wide-reaching, possibly surprising, language feature in just for that.
//      I personally like having the structs composing the `Entity` struct be a sort of "superset" or "skin" that one must 
//      have to (very lightly) drill into to get the underlying lower fields. It is sort of weird to be able to directly
//      access the `Entity` fields on a player, but also sort of not. For now, an `id` getter is auto-implemented for
//      the id field, and other fields may follow. If others follow, it could be seen as basically brute forcing the
//      Deref approach. They're both the same shit in the end I guess, but this approach feels cleaner in some sense,
//      especially in the manner it can be built upon to create certain expressions (e.g. "all entities" v.s. 
//      "all things that give entities"), although I incur some OOP-isms (getter spam). Can't win 'em all.
//    - It's not semantically a smart pointer at all, but although the docs once used to discourage `Deref(Mut)`
//      implementation on non-smart pointer types, that is no longer the case. But still, it is a good bit off-key.
//      `DerefMut<Target=Entity>` is not necessarily saying the same thing as `IsEntity`. One says they can give
//      references to `Entity`, the other is saying that they *are* an entity. I can't implement `DerefMut<Target=Entity>`
//      on the `Entity` type directly either due to recursive reasons. It may be useful in the future to be able to directly
//      talk about all entities including the `Entity` struct itself in the future, without having to reintroduce the type
//      merely as a flag to signal exactly that, but at that point you may as well just have all functionally also fall under
//      the flag, saving on strange boilerplate and also being nicer in semantic expression. Still, it's ultimately a design choice.
//      Both manners will eventually give very similar looking results -- all expression, even if slightly different in form, look,
//      and/or applicationn, can be expressed.
pub trait IsEntity {
    fn ref_entity(&self) -> &Entity;
    fn mut_entity(&mut self) -> &mut Entity;

    fn id(&self) -> usize {
        self.ref_entity().id
    }
}

impl IsEntity for Entity {
    fn ref_entity(&self) -> &Entity {
        self
    }
    fn mut_entity(&mut self) -> &mut Entity {
        self
    }
}

impl IsEntity for Player {
    fn ref_entity(&self) -> &Entity {
        &self.entity
    }
    fn mut_entity(&mut self) -> &mut Entity {
        &mut self.entity
    }
}

impl IsEntity for NPC {
    fn ref_entity(&self) -> &Entity {
        &self.entity
    }
    fn mut_entity(&mut self) -> &mut Entity {
        &mut self.entity
    }
}

pub struct EntityBuilder {
    eid_count: usize
}

impl EntityBuilder {
    pub fn new() -> Self {
        EntityBuilder { eid_count: 0 }
    }

    pub async fn build_entity_from_center(
        &mut self,
        (x, y): (f32, f32),
        (bwidth, bheight): (f32, f32),
        draw_size: Vec2,
        rotation: f32,
        texture_path: &str,
    ) -> GResult<Entity> {
        let e = Entity::build_from_center(
            (x, y),
            (bwidth, bheight),
            draw_size,
            rotation,
            texture_path,
            self.eid_count,
        );
        self.eid_count += 1;
        e.await
    }

    pub async fn build_entity_from_boundary(
        &mut self,
        boundary: Rect,
        draw_size: Vec2,
        rotation: f32,
        texture_path: &str,
    ) -> GResult<Entity> {
        let e = Entity::build_from_boundary(
            boundary,
            draw_size,
            rotation,
            texture_path,
            self.eid_count,
        );
        self.eid_count += 1;
        e.await
    }

    pub async fn init_player(&mut self) -> GResult<Player> {
        let p_entity = self.build_entity_from_center(
            (0.0, 0.0),
            (PLAYER_SIZE.x, PLAYER_SIZE.y),
            PLAYER_SIZE,
            0.0, 
            PLAYER_SPRITE_PATH
        ).await?;
        Ok(Player::from_entity(p_entity))
    }
    
    pub async fn init_npcs(&mut self) -> GResult<Vec<NPC>> {
        let npc1_entity = self.build_entity_from_center(
            (300.0, 360.0),
            (70.0, 70.0), 
            vec2(120.0, 120.0),
            0.0,
            "assets/sprites/test_npc.png",
            
        ).await?;
        let npc1 = NPC::build_from_entity(
            npc1_entity,
            "assets/dialogue/test_dialogue.txt"
        ).await?;

        Ok(vec![npc1])
    }
}

pub struct EntityManager {
    pub player: Player,
    pub npcs: Vec<NPC>,
}

impl EntityManager {
    pub fn from_parts(player: Player, npcs: Vec<NPC> ) -> Self {
        EntityManager { player, npcs }
    }

    
    /// Draws all loaded/enabled entities
    pub fn draw_loaded(&self) {
        self.npcs.iter().for_each(NPC::draw);
        self.player.draw();
    }

    pub fn ref_player(&self) -> &Player {
        &self.player
    }

    pub fn mut_player(&mut self) -> &mut Player {
        &mut self.player
    }

    pub fn ref_npcs(&self) -> &[NPC] {
        &self.npcs
    }

    pub fn mut_npcs(&mut self) -> &mut [NPC] {
        &mut self.npcs
    }
}