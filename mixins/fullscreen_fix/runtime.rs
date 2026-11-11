// Fullscreen Fix: on macOS, the game entering exclusive fullscreen at launch
// switches the display mode but keeps drawing for the desktop's size, so the
// picture fills only part of the screen (black bands right and below) until
// fullscreen is turned off and on again. The window's own size is stale too,
// so it can't be detected from the size: this does that off-and-on once, a
// moment after launch, as F11 twice would.
use std::cell::Cell;
use std::time::{Duration, Instant};

/// The mod's id, name and version.
pub const ID: &str = "fullscreen_fix";
pub const NAME: &str = "Fullscreen Fix";
pub const VERSION: &str = "0.2.1";
pub const DESCRIPTION: &str =
    "On macOS, re-enters exclusive fullscreen once after launch, so the picture fills the screen.";

/// How long after the window is first seen in exclusive fullscreen it leaves it,
/// and how long after that it enters it again (macOS animates both).
const SETTLE: Duration = Duration::from_millis(1500);
const OUT: Duration = Duration::from_millis(1500);

#[derive(Clone, Copy)]
enum Stage {
    /// Waiting for the window to be in exclusive fullscreen.
    Waiting,
    /// Seen in exclusive fullscreen at this instant.
    Seen(Instant),
    /// Turned fullscreen off at this instant.
    Off(Instant),
    /// Done for this launch.
    Done,
}

thread_local! {
    static STAGE: Cell<Stage> = const { Cell::new(Stage::Waiting) };
}

pub(super) fn frame(game: &mut crate::game::Game<'_>) {
    // Only macOS has the stale-size bug: elsewhere the mod leaves fullscreen alone.
    if !cfg!(target_os = "macos") {
        return;
    }
    // Split-screen guests size their own windows.
    if game.options.split_guest {
        return;
    }
    let Some(window) = &game.window else { return };
    let exclusive = matches!(
        window.fullscreen(),
        Some(winit::window::Fullscreen::Exclusive(_))
    );
    let now = Instant::now();
    match STAGE.get() {
        Stage::Waiting if exclusive => STAGE.set(Stage::Seen(now)),
        Stage::Seen(at) if now - at >= SETTLE => {
            if !exclusive {
                // The player left fullscreen on their own: nothing to redo.
                STAGE.set(Stage::Done);
                return;
            }
            log::info!("Fullscreen Fix: leaving exclusive fullscreen to enter it again");
            game.toggle_fullscreen();
            STAGE.set(Stage::Off(now));
        }
        Stage::Off(at) if now - at >= OUT => {
            if !game.settings.fullscreen {
                game.toggle_fullscreen();
            }
            STAGE.set(Stage::Done);
        }
        _ => {}
    }
}
