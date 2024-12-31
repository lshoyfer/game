use game::prelude::*;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Game".to_owned(),
        high_dpi: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut player = Player::new(VIRTUAL_WIDTH / 2.0, VIRTUAL_HEIGHT / 2.0);
    loop {
        let screen_w = screen_width();        
        let screen_h = screen_height();
        let dpi_scale = screen_dpi_scale();

        let mut camera = build_camera(screen_w, screen_h, dpi_scale);
        debug!("{:#?}", &camera);

        let dt = get_frame_time();
        player.update(dt);
        camera.target = player.position;

        clear_background(BLACK);
        set_camera(&camera);
        draw_text("Camera Reset", 20.0, 40.0, 30.0, RED);
        draw_rectangle_lines(0.0, 0.0, VIRTUAL_WIDTH, VIRTUAL_HEIGHT, 2.0, WHITE);
        player.draw();

        set_default_camera();
        next_frame().await
    }
}
