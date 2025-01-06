use crate::prelude::*;
/// Partially implementable.
/// Err value indicates unimplemented.
/// Ok value comes with the current value
/// that was toggled to.
pub trait Debuggable {
    fn toggle_hitbox(&mut self) -> Result<bool, ()> {
        Err(())
    }
}

impl<T: IsEntity> Debuggable for T{
    fn toggle_hitbox(&mut self) -> Result<bool, ()> {
        let this = self.mut_entity();
        this.show_hitbox = !this.show_hitbox;
        Ok(this.show_hitbox)
    }
}