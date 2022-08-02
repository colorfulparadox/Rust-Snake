use bevy::{prelude::*, sprite::Anchor, asset};

use crate::*;

const TEXTURE_ATLAS_PATH: &str = "Images/texture_atlas.png";
const CHECKERBOARD_PATH: &str = "Images/checkerboard.png";

const BOLD_PATH: &str = "Fonts/GravityBold.ttf";
const REG_PATH: &str = "Fonts/GravityRegular.ttf";

pub struct GameTextures {
    pub snake_atlas: Handle<TextureAtlas>,
    pub checkerboard: Handle<Image>
}

pub struct GameFonts {
    pub bold: Handle<Font>,
    pub regular: Handle<Font>
}

pub fn create(
    mut commands: Commands, 
    mut app_state: ResMut<State<AppState>>,
    asset_server: Res<AssetServer>, 
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {

    let texture_handle: Handle<Image> = asset_server.load(TEXTURE_ATLAS_PATH);
    let texture_atlas: TextureAtlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.), 4, 1);
    let textures: GameTextures = GameTextures {
        snake_atlas: texture_atlases.add(texture_atlas),
        checkerboard: asset_server.load(CHECKERBOARD_PATH),
    };
    commands.insert_resource(textures);

    
    let fonts: GameFonts = GameFonts {
        bold: asset_server.load(BOLD_PATH),
        regular: asset_server.load(REG_PATH),
    };
    commands.insert_resource(fonts);

    app_state.set(AppState::Menu).unwrap();
}