# Fullscreen Fix

A native mod for [Horizon](https://github.com/horizon-rust/horizon) that makes
exclusive fullscreen fill the screen on macOS.

## The bug

When the game starts in exclusive fullscreen at a resolution other than the
desktop's (for example 1920x1200 on a Retina display), macOS switches the
display mode but the game keeps drawing for the old size. The picture covers
only the top-left part of the screen, with black bands to the right and below,
until fullscreen is turned off and on again by hand. The window reports the old
size too, so the game can't tell anything is wrong.

Exclusive fullscreen at a lower resolution is worth keeping: with a shader pack
it can run several times faster than borderless at the display's native size.

## What the mod does

About 1.5 s after the game first shows in exclusive fullscreen, it leaves
fullscreen and enters it again 1.5 s later, as pressing F11 twice would. The
screen blinks once at launch and the picture then fills it. It does this once
per launch, only in exclusive fullscreen, and does nothing if you have already
left fullscreen yourself. Split-screen guest windows are left alone. On Windows
and Linux, which don't have the bug, it does nothing.

It shows in Options > Mods as "Fullscreen Fix".

## Building

It is a horizon-mixins bundle pinned to Horizon `fdc87ae` (Nightly 2026-11-08),
because it reads the game's window and settings and calls its own fullscreen
toggle. Install the `.hnmod` from the registry or the
[releases](https://github.com/voltzz900/horizon-fullscreen-fix/releases), turn it
on, and build natives in the launcher.

`./release.sh` packs `mixins/fullscreen_fix/` into `dist/fullscreen_fix-<version>.hnmod`.

## License

MIT, see [LICENSE](LICENSE).
