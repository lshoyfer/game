use game::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Game".to_owned(),
        high_dpi: true,
        fullscreen: true,
        ..Default::default()
    }
}

// TODO
// FIXME
// BUG
// REVIEW
// TSC
//IDEA //  

#[macroquad::main(window_conf)]
async fn main() -> GResult<()> {
    let mut player = Player::build_from_center(
        (0.0, 0.0),
        (PLAYER_SIZE.x, PLAYER_SIZE.y),
        PLAYER_SIZE,
        0.0, 
        PLAYER_SPRITE_PATH
    ).await?;
    let mut npc1 = NPC::build_from_center(
        (300.0, 360.0),
        (70.0, 70.0), 
        vec2(120.0, 120.0),
        0.0,
        "assets/sprites/test_npc.png",
        "assets/dialogue/test_dialogue.txt", 
    ).await?;
    let _ = player.toggle_hitbox();
    let _ = npc1.toggle_hitbox();

    loop {

        let screen_w = screen_width();        
        let screen_h = screen_height();
        let dpi_scale = screen_dpi_scale();

        let mut camera = build_camera(screen_w, screen_h, dpi_scale);
        dlog!(Level::Trace, &camera);

        handle_player_updates_and_collisions(&mut player, &mut npc1);
        camera.target = player.position();

        clear_background(WHITE);
        set_camera(&camera);
        draw_text("Camera Reset", 20.0, 40.0, 30.0, RED);
        draw_rectangle_lines(0.0, 0.0, VIRTUAL_WIDTH, VIRTUAL_HEIGHT, 2.0, BLACK);

        npc1.draw();        
        player.draw();

        set_default_camera();
        next_frame().await
    }
}

fn handle_player_updates_and_collisions(player: &mut Player, npc: &mut NPC) {
    let dt = get_frame_time();

    player.update_x(dt);
    if player.overlaps_excluding_bounds(npc) {
        if player.ref_velocity().x.is_sign_positive() {
            player.move_by_boundary_to(vec2(npc.ref_boundary().left() - player.bsize().x, player.ref_boundary().y));
        } else {
            player.move_by_boundary_to(vec2(npc.ref_boundary().right(), player.ref_boundary().y));
        }
    }

    player.update_y(dt);
    if player.overlaps_excluding_bounds(npc) {
        if player.ref_velocity().y.is_sign_positive() {
            player.move_by_boundary_to(vec2(player.ref_boundary().x, npc.ref_boundary().top() - player.bsize().y));
        } else {
            player.move_by_boundary_to(vec2(player.ref_boundary().x, npc.ref_boundary().bottom()));
        }
    }
}
