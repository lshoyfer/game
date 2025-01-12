use crate::prelude::*;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Game".to_owned(),
        high_dpi: true,
        fullscreen: false,
        ..Default::default()
    }
}

pub async fn init(eb: &mut EntityBuilder) -> GResult<Game> {
    let player = eb.init_player().await?;
    let npcs = eb.init_npcs().await?;
    let em = EntityManager::from_parts(player, npcs);
    let font = load_ttf_font(DEFAULT_FONT_TTF_PATH).await?;
    let dm = DialogueManager::from_text_params(
        OwnedTextParams {
            font: Some(font),
            font_size: DEFAULT_FONT_SIZE,
            color: DEFAULT_FONT_COLOR,
        }
    );
    let um = DrawManager::new();
    Ok(Game::from_parts(em, dm, um))
}

pub fn draw_basic_testing_pattern() {
    for i_x in (0..(LOGICAL_WIDTH as i32)).step_by((LOGICAL_WIDTH / 100.0) as usize) {
        for i_y in (0..(LOGICAL_HEIGHT as i32)).step_by((LOGICAL_HEIGHT / 100.0) as usize) {
            // dbg!(i_x, i_y);
            draw_circle(i_x as f32, i_y as f32, 1.0, WHITE);
        }
    }
}