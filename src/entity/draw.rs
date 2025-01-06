use crate::prelude::*;

pub trait Drawable {
    fn draw(&self);
}

impl<T: IsEntity> Drawable for T {
    fn draw(&self) {
        let this = self.ref_entity();
        let params = DrawTextureParams {
            dest_size: Some(this.draw_size),
            rotation: this.rotation,
            ..Default::default()
        };
        let tl = this.boundary.center() - this.draw_size / 2.0; 
        draw_texture_ex(&this.texture, tl.x, tl.y, WHITE, params);

        if this.show_hitbox {
            let Rect { x, y, w, h } = this.boundary;
            draw_rectangle_lines(x, y, w, h, 4.0, BLACK);
        }
    }
}