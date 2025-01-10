use crate::prelude::*;

/// Handles **Physical** Pixels
#[derive(Clone, Copy)]
pub struct VResTranslationVals {
    pub zoom: Vec2,
    /// Physical Pixels, non-floating because of camera viewport requirements
    pub origin: IVec2,
    /// Physical Pixels, non-floating because of camera viewport requirements
    pub dimensions: IVec2,
}

/// Handles **Logical** Pixels
#[derive(Debug, Clone, Copy)]
pub struct LetterboxDimensions {
    pub top: Rect,
    pub bottom: Rect,
    pub left: Rect,
    pub right: Rect
}

#[derive(Clone, Copy)]
pub struct LogicalWindowData {
    pub width: f32,
    pub height: f32,
    pub dpi_scale: f32,
}

/// All public method calls begin a "lifecycle" for the manager if not already started, which is 
/// essentially an expression for saying the UIManager has calculated and cached important
/// state for the current frame.
/// 
/// ## Important Requirement
/// You must flush the state and end the lifecycle via [`UIManager::lifecycle_flush_and_end`]
/// before the end of every frame and after all other method calls.
/// This is important and required because otherwise you will reuse old frame data and may experience strange UI.
#[derive(Default)]
pub struct UIManager {
    pub in_lifecycle: bool,
    pub curr_window_data: Option<LogicalWindowData>,
    pub curr_vres_trans: Option<VResTranslationVals>,
    pub curr_letterboxing: Option<LetterboxDimensions>,
}

impl UIManager {
    pub fn new() -> UIManager {
        UIManager { ..Default::default() }
    }

    /// Begins a lifecycle if doesn't exist and calculates state, returning it. Otherwise, returns current lifecycle state.
    fn lifecycle_calc_or_get_states(&mut self) -> (&LogicalWindowData, &VResTranslationVals, &LetterboxDimensions)  {
        // SAFETY: We trivially know all relevant places are in the Some variant before calling unwrap_unchecked if
        // in_lifecycle is false. Otherwise, there is a held invariant that in_lifecycle is only true if set from
        // that if statement, which first ensures all fields are Some() before setting in_lifecycle to true.
        unsafe {
            if !self.in_lifecycle {
                self.curr_window_data = Some(UIManager::get_logical_window_data());
                self.curr_vres_trans = Some(UIManager::calculate_virtual_camera_trans_vals(self.curr_window_data.unwrap_unchecked()));
                self.curr_letterboxing = Some(UIManager::calculate_letterbox_dimensions(self.curr_window_data.unwrap_unchecked(), self.curr_vres_trans.unwrap_unchecked()));
                self.in_lifecycle = true;
            }
            (self.curr_window_data.as_ref().unwrap_unchecked(), self.curr_vres_trans.as_ref().unwrap_unchecked(), self.curr_letterboxing.as_ref().unwrap_unchecked())
        }
    }

    /// Should be drawn with default camera for expected behavior 
    /// ## Side-Effect
    /// Begins a lifecycle if not currently in one.
    pub fn draw_letterboxing_absolute(&mut self) {
        let (_, _, LetterboxDimensions { top, bottom, left, right}) = self.lifecycle_calc_or_get_states();
        draw_rectangle(top.x, top.y, top.w, top.h, BLACK);
        draw_rectangle(bottom.x, bottom.y, bottom.w, bottom.h, BLACK);
        draw_rectangle(left.x, left.y, left.w, left.h, BLACK);
        draw_rectangle(right.x, right.y, right.w, right.h, BLACK);
    }

    /// ## Side-Effect
    /// Begins a lifecycle if not currently in one.
    pub fn build_player_camera(&mut self, player_position: Vec2) -> Camera2D {
        let (_, &VResTranslationVals { zoom, origin, dimensions }, _) = self.lifecycle_calc_or_get_states();
        Camera2D {
            zoom,
            target: player_position,
            viewport: Some((
                origin.x,
                origin.y,
                dimensions.x,
                dimensions.y,
            )),
            ..Default::default()
        }
    }
    
    /// ## Important Requirement
    /// Call before frame end and after any other method to end the lifecycle and flush inner state.
    /// This is important and required because otherwise you will reuse old frame data and may experience strange UI.
    ///
    /// Is a no-op if wasn't in a lifecycle.
    pub fn lifecycle_flush_and_end(&mut self) {
        if self.in_lifecycle {
            self.curr_window_data = None;
            self.curr_vres_trans = None;
            self.curr_letterboxing = None;
            self.in_lifecycle = false;
        }
    }

    /// Should be drawn with default camera for expected behavior 
    /// ## Side-Effect
    /// Begins a lifecycle if not currently in one.
    pub fn draw_dialogue_frame_absolute(&mut self) {
        let (&LogicalWindowData { width: screen_w, height: screen_h, .. }, _, _) = self.lifecycle_calc_or_get_states();
        let d_frame_height = screen_h / 5.0;
        draw_rectangle(0.0, screen_h - d_frame_height, screen_w, d_frame_height, WHITE);
        draw_rectangle_lines(0.0, screen_h - d_frame_height, screen_w, d_frame_height, 3.0, GRAY);
    }

    /// Should be drawn with default camera for expected behavior 
    /// ## Side-Effect
    /// Begins a lifecycle if not currently in one.
    pub fn draw_dialogue_text_absolute(&mut self, text: &str) {
        let (LogicalWindowData { height: screen_h, .. }, _, _) = self.lifecycle_calc_or_get_states();
        draw_text(text, 0.0, screen_h - screen_h / 5.0 + 100.0, 50.0, BLACK);
    }

    fn calculate_letterbox_dimensions(window_data: LogicalWindowData, vres_trans_vals: VResTranslationVals) -> LetterboxDimensions {
        let Vec2 { x: v_origin_y, y: v_dimension_y } = vec2(
            vres_trans_vals.origin.y as f32, 
            vres_trans_vals.dimensions.y as f32
        ) / screen_dpi_scale(); // draw calls use logical pixels
        let top = Rect::new(0.0, 0.0, window_data.width, v_origin_y);
        let bottom = Rect::new(0.0, v_origin_y + v_dimension_y, window_data.width, v_origin_y);

        let Vec2 { x: v_origin_x, y: v_dimension_x} = vec2(
            vres_trans_vals.origin.x as f32, 
            vres_trans_vals.dimensions.x as f32
        ) / screen_dpi_scale(); // draw calls use logical pixels
        let left = Rect::new(0.0, 0.0, v_origin_x, window_data.height);
        let right = Rect::new(v_origin_x + v_dimension_x, 0.0, v_origin_x, window_data.height);

        LetterboxDimensions { top, bottom, left, right }
    }

    fn calculate_virtual_camera_trans_vals(window_data: LogicalWindowData) -> VResTranslationVals {
        let LogicalWindowData { width: screen_w, height: screen_h, dpi_scale } = window_data;
        let v_scale_x = screen_w / VIRTUAL_WIDTH;
        let v_scale_y = screen_h / VIRTUAL_HEIGHT;
        let v_scale = v_scale_x.min(v_scale_y);

        let offset_x = (screen_w - VIRTUAL_WIDTH * v_scale) / 2.0 * dpi_scale;
        let offset_y = (screen_h - VIRTUAL_HEIGHT * v_scale) / 2.0 * dpi_scale;

        let scaled_w = VIRTUAL_WIDTH * v_scale * dpi_scale;
        let scaled_h = VIRTUAL_HEIGHT * v_scale * dpi_scale;

        // Camera has an orthographic rendering zoom API with a clip space of [-1, 1], 
        // such that we must input the translation from our virtual dimensions into 
        // normalized ones as the "zoom" values. So we divide 2 ("2 units" of clip space)
        // by each respective dimension for the ratio/translation value. Also, it uses 
        // logical pixels. Don't ask me why.
        let zoom_x = 2.0 / VIRTUAL_WIDTH;
        let zoom_y = 2.0 / VIRTUAL_HEIGHT;

        VResTranslationVals { 
            zoom: vec2(zoom_x, zoom_y), 
            origin: ivec2(offset_x.round() as i32, offset_y.round() as i32), 
            dimensions: ivec2(scaled_w.round() as i32, scaled_h.round() as i32)
        }
    }

    fn get_logical_window_data() -> LogicalWindowData {
        LogicalWindowData { width: screen_width(), height: screen_height(), dpi_scale: screen_dpi_scale() }
    }
}
