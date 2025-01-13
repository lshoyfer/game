use crate::prelude::*;

pub trait Teleportable {
    /// `move_by_origin_to` is equivalent to its `center_to` counterpart for circle boundaries
    fn move_by_origin_to(&mut self, destination: Vec2);
    /// `move_by_center_to` is equivalent to its `origin_to` counterpart for circle boundaries
    fn move_by_center_to(&mut self, destination: Vec2);
    fn offset(&mut self, offset: Vec2);
}

pub trait Moveable {
    fn velocity(&self) -> Vec2;
    fn ref_velocity(&self) -> &Vec2;
    fn mut_velocity(&mut self) -> &mut Vec2;
}