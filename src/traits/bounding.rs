use crate::prelude::*;

pub trait RectBounded {
    fn ref_boundary(&self) -> &Rect;
    fn mut_boundary(&mut self) -> &mut Rect; 

    fn boundary(&self) -> Rect {
        *self.ref_boundary()
    } 

    fn intersect(&self, other: &impl RectBounded) -> Option<Rect> {
        Rect::intersect(self.ref_boundary(), other.boundary())
    }

    fn overlaps(&self, other: &impl RectBounded) -> bool {
        Rect::overlaps(self.ref_boundary(), other.ref_boundary())
    }

    fn overlaps_excluding_bounds(&self, other: &impl RectBounded) -> bool  {
        let this = self.ref_boundary();
        let other = other.ref_boundary();
        this.left() < other.right()
            && this.right() > other.left()
            && this.top() < other.bottom()
            && this.bottom() > other.top()
    }

    fn position(&self) -> Vec2 {
        self.ref_boundary().center()
    }

    fn bsize(&self) -> Vec2 {
        self.ref_boundary().size()
    }
}
