use crate::prelude::*;


pub struct UIManager;

impl UIManager {
    pub fn new() -> UIManager {
        UIManager
    }
    pub fn draw_game_borders() {
        draw_rectangle_lines(0.0, 0.0, screen_width(), screen_height(), 1.0, GRAY);
    }

    pub fn draw_dialogue_frame() {
        draw_rectangle(0.0, screen_height() - screen_height() / 5.0, screen_width(), screen_height(), WHITE);
        draw_rectangle_lines(0.0, screen_height() - screen_height() / 5.0, screen_width(), screen_height(), 3.0, GRAY);
    }

    pub fn draw_dialogue(text: &str) {
        draw_text(text, 0.0, screen_height() - screen_height() / 5.0 + 100.0, 50.0, BLACK);
    }
}
