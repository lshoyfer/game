use game::prelude::*;

#[macroquad::main(window_conf)]
async fn main() -> GResult<()> {
    env_logger::try_init()?;

    let mut eb = EntityBuilder::new();
    let mut g = init(&mut eb).await?;

    dlog!(Level::Info, "ENTITY IDS\n\tPLAYER: {:?}\n\tNPCS: {:?}",
        g.em.ref_player().id(), 
        g.em.ref_npcs().iter().map(NPC::id).collect::<Vec<_>>()
    );

    loop {
        set_default_camera(); // Not strictly necessary if the end flushes it (it does) but avoids confusion during development
        clear_background(WHITE);
        g.init_view_and_update_entites();
        g.draw_loaded_entites();
        g.handle_ui();
        next_frame().await
    }
}