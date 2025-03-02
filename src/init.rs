use crate::prelude::*;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Game".to_owned(),
        high_dpi: true,
        fullscreen: false,
        ..Default::default()
    }
}

impl Game {
    pub async fn init() -> GResult<Game> {
        let mut eb = EntityBuilder::new();
        let player = eb.init_player().await?;
        let npcs = eb.init_npcs().await?;

        let em = EntityManager { player, npcs };

        let font = load_ttf_font(DEFAULT_FONT_TTF_PATH).await?;
        let dm = DialogueManager::from_text_params(
            OwnedTextParams {
                font: Some(font),
                font_size: DEFAULT_FONT_SIZE,
                color: DEFAULT_FONT_COLOR,
            }
        );

        let pset = PSet::current();

        let test_map = read_map_file("assets/maps/test_map").await?;

        Ok(Game { eb, em, dm, pset, map: test_map, geometry_textures: HashMap::new(), })
    }
}