pub trait Updateable {
    fn update_x(&mut self, dt: f32);
    fn update_y(&mut self, dt: f32);

    fn update(&mut self, dt: f32) {
        self.update_x(dt);
        self.update_y(dt);
    }
}
