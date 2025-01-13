use crate::prelude::*;

pub fn set_player_camera(pset: &PSet, player_position: Vec2) {
    let Viewport { x, y, w, h } = pset.physical.viewport_from_logical;
    let player_camera = Camera2D {
        zoom: pset.logical.camera_zoom,
        target: player_position,
        viewport: Some((x, y, w, h)),
        ..Default::default()
    };
    set_camera(&player_camera);
}

/// Equivalent in end function to [`set_default_camera`] but changes the viewport from previous camera to
/// what it should be (https://github.com/not-fl3/macroquad/commit/a66852aa9d7efa86656c477825abe3a5072f8a1e).
pub fn set_natural_camera(pset: &PSet) {
    let Viewport { x, y, w, h } = pset.physical.viewport_from_natural; 
    let (zoom, target) = (pset.natural.camera_zoom, pset.natural.camera_target);
    let natural_camera = Camera2D {
        zoom,
        target,
        viewport: Some((x, y, w, h)),
        ..Default::default()
    };
    set_camera(&natural_camera);
}

pub fn set_ui_camera(pset: &PSet) {
    let Viewport { x, y, w, h } = pset.physical.viewport_from_logical; 
    let (zoom, target) = (pset.logical.camera_zoom, pset.logical.camera_target);
    let natural_camera = Camera2D {
        zoom,
        target,
        viewport: Some((x, y, w, h)),
        ..Default::default()
    };
    set_camera(&natural_camera);
}