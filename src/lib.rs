#![allow(unused)]

use bevy::{prelude::*, window::WindowCloseRequested};
use bevy::app::AppExit;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

mod resources;
mod menu;
mod game;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    None,
    InGame,
    Dead,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    Loading,
    Menu,
    Game,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Settings {
    game_name: String,
    verison: f32,
    window_size: WindowSize,
    resizable: bool,
    tile_size: i32,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct WindowSize {
    width: f32,
    height: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PlayerData {
    highscore: i32
}

const SETTINGS_PATH: &str = "data/settings.json";
const GAME_ICON_PATH: &str = "build/windows/game_icon.png";
const PLAYER_DATA_PATH: &str = "data/player_data.json";

pub struct MainGamePlugin;

impl Plugin for MainGamePlugin {
    fn build(&self, app: &mut App) {

        let settings: Settings = load_json::<Settings>(SETTINGS_PATH);
        let player_data: PlayerData = load_json::<PlayerData>(PLAYER_DATA_PATH);

        //println!("{settings:#?}");
        //println!("{player_data:#?}");

        app        
            .add_startup_system(setup)
            .add_startup_system(set_window_icon)
            .add_startup_system(resources::create)
            
            .insert_resource(WindowDescriptor {
                width: settings.window_size.width,
                height: settings.window_size.height,
                title: String::from(settings.game_name.as_str()),
                resizable: settings.resizable,
                ..Default::default()
            })

            .add_plugins(DefaultPlugins)
            .add_plugin(menu::MainMenuPlugin)
            .add_plugin(game::GamePlugin)
            
            .add_state(AppState::Loading)
            .add_state(GameState::None)

            .insert_resource(settings)
            .insert_resource(player_data)
            .insert_resource(ClearColor(Color::rgb(0.02, 0.04, 0.08)))

            .add_system(on_window_close);
    }
} 

fn load_json<T>(path: &str) -> T where T: serde::de::DeserializeOwned {
    let json_file_path = Path::new(path);
    let file = File::open(json_file_path);
    let reader = BufReader::new(file.unwrap());
    let json_data: T = serde_json::from_reader(reader).expect("JSON for was not formatted correctly!");

    json_data
}

fn setup(
    mut commands: Commands
) {
    commands.spawn_bundle(UiCameraBundle::default());

    //camera
    let mut camera = OrthographicCameraBundle::new_2d();
     camera.transform.translation = Vec3::new( 240., 240., 2.);
    
     camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;
    
    commands.spawn_bundle(camera);
}

/*
    A hacky way to set window icon from: https://bevy-cheatbook.github.io/window/icon.html
    winit = "0.26.1"
    image = "0.23.14" (https://crates.io/crates/image)
 */
use bevy::winit::WinitWindows;

fn set_window_icon(windows: NonSend<WinitWindows>) {
    use bevy::window::WindowId;
    use winit::window::Icon;
    use image::GenericImageView;

    let primary = windows.get_window(WindowId::primary()).unwrap();

    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(GAME_ICON_PATH)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    primary.set_window_icon(Some(icon));
}

fn on_window_close(
    app_exit_event: EventReader<AppExit>,
    mut player_data: ResMut<PlayerData>
) {
    if app_exit_event.is_empty() { return }

    std::fs::write(
        Path::new(PLAYER_DATA_PATH),
        serde_json::to_string_pretty(player_data.as_ref()).unwrap(),
    ).unwrap();

}