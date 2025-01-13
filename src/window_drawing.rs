use crate::prelude::*;

/// Should be drawn with ui camera ([`set_ui_camera`]) for expected behavior.
pub fn draw_dialogue_frame(pset: &PSet) {
    let PxWindow { width: logical_w, height: logical_h } = pset.logical.window;
    let d_frame_height = logical_h / 5.0;
    draw_rectangle(0.0, logical_h - d_frame_height, logical_w, d_frame_height, WHITE);
    draw_rectangle_lines(0.0, logical_h - d_frame_height, logical_w, d_frame_height, 3.0, GRAY);
}

/// Should be drawn with ui camera ([`set_ui_camera`]) for expected behavior.
pub fn draw_dialogue_text(pset: &PSet, text: &str, maybe_params: Option<TextParams>) {
    let logical_h = pset.logical.window.height;
    if let Some(params) = maybe_params {
        dlog!(Level::Info, draw_text_ex(text, 0.0, logical_h - logical_h / 5.0 + params.font_size as f32, params));
    } else {
        // use default ttf font -- we will probably never have a None maybe_params but this is here b/c easy to implement
        draw_text(text, 0.0, logical_h - logical_h / 5.0 + DEFAULT_FONT_SIZE as f32, DEFAULT_FONT_SIZE as f32, DEFAULT_FONT_COLOR);
    }
}

/// Should be drawn with natural camera [`set_natural_camera`] for expected behavior 
pub fn draw_letterboxing_natural(pset: &PSet) {
    let LetterboxDimensions { top, bottom, left, right } = pset.natural.letterbox;
    draw_rectangle(top.x, top.y, top.w, top.h, BLACK);
    draw_rectangle(bottom.x, bottom.y, bottom.w, bottom.h, BLACK);
    draw_rectangle(left.x, left.y, left.w, left.h, BLACK);
    draw_rectangle(right.x, right.y, right.w, right.h, BLACK);
}