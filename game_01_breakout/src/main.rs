#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::prelude::*;

fn main() {
    // Check the TARGET, not the host — #[cfg(target_os = "windows")]
    // evaluates against the host in build scripts and silently does
    // nothing when you cross-compile from Linux.
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icon.ico"); // must be .ico, not .png
        res.compile().unwrap();
    }

    App::new().add_plugins(game_01_breakout::plugin).run();
}
