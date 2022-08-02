#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;

use snake::MainGamePlugin;

fn main() { 
    App::new()
        .add_plugin(MainGamePlugin)
        .run();
}