use game::prelude::*;

#[macroquad::main(window_conf)]
async fn main() -> GResult<()> {
    let mut eb = EntityBuilder::new();
    let mut g = init(&mut eb).await?;

    dlog!(Level::Info, "ENTITY IDS\n\tPLAYER: {:?}\n\tNPCS: {:?}",
        g.em.ref_player().id(), 
        g.em.ref_npcs().iter().map(NPC::id).collect::<Vec<_>>()
    );

    loop {
        let screen_w = screen_width();        
        let screen_h = screen_height();
        let dpi_scale = screen_dpi_scale();

        let mut camera = build_camera(screen_w, screen_h, dpi_scale);
        dlog!(Level::Trace, &camera);

        g.handle_updates_and_collisions();
        camera.target = g.em.ref_player().position();

        clear_background(WHITE);
        set_camera(&camera);
        draw_text("Camera Reset", 20.0, 40.0, 30.0, RED);
        draw_rectangle_lines(0.0, 0.0, VIRTUAL_WIDTH, VIRTUAL_HEIGHT, 2.0, BLACK);

        g.em.draw_loaded();
        g.handle_ui();

        set_default_camera();
        next_frame().await
    }
}