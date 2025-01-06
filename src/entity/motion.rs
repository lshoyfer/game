use crate::prelude::*;

pub trait Teleportable {
    fn move_by_boundary_to(&mut self, destination: Vec2);
    fn move_by_center_to(&mut self, destination: Vec2);
    fn offset(&mut self, offset: Vec2);
}

impl<T: IsEntity> Teleportable for T {
    fn move_by_boundary_to(&mut self, destination: Vec2) {
        self.mut_entity().boundary.move_to(destination);
    }

    fn move_by_center_to(&mut self, destination: Vec2) {
        let boundary = &mut self.mut_entity().boundary;
        boundary.move_to(destination - boundary.size() / 2.0);
    }

    fn offset(&mut self, offset: Vec2) {
       self.mut_boundary().x += offset.x;
       self.mut_boundary().y += offset.y;
    }
}

pub trait Moveable {
    fn velocity(&self) -> Vec2;
    fn ref_velocity(&self) -> &Vec2;
    fn mut_velocity(&mut self) -> &mut Vec2;
}

impl<T: IsEntity> Moveable for T {
    fn velocity(&self) -> Vec2 {
        self.ref_entity().velocity
    }

    fn ref_velocity(&self) -> &Vec2 {
        &self.ref_entity().velocity
    }
    
    fn mut_velocity(&mut self) -> &mut Vec2 {
        &mut self.mut_entity().velocity
    }
}