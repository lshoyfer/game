use game::prelude::*;

#[macroquad::main(window_conf)]
async fn main() -> GResult<()> {
    env_logger::try_init()?;

    let mut g = Game::init().await?;

    dlog!(Level::Info, "ENTITY IDS\n\tPLAYER: {:?}\n\tNPCS: {:?}",
        g.em.ref_player().id(), 
        g.em.ref_npcs().iter().map(NPC::id).collect::<Vec<_>>()
    );

    loop {
        g.start_frame();
        g.init_player_view_and_update_entites();
        g.draw_loaded_entites();
        g.handle_ui();
        g.next_frame().await;
    }
}