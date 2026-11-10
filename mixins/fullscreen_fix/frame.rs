// Once a frame: the surface kept at the window's size.
fn fullscreen_fix_frame(game: &mut crate::game::Game<'_>) {
    crate::game::fullscreen_fix::frame(game);
}
