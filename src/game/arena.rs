use bevy::prelude::*;

use crate::*;

#[derive(Component)]
pub struct Arena;

pub const ARENA_SIZE: f32 = 480.;

pub fn setup(
    mut commands: Commands,
    textures: Res<resources::GameTextures>, 
    //settings: Res<Settings>, 
) {
    commands.spawn_bundle(SpriteBundle {
        texture: textures.checkerboard.clone(),
        sprite: Sprite {
            //anchor: bevy::sprite::Anchor::TopLeft,
            custom_size: Some(Vec2::new(480.,480.)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(240.,240.,0.),
            ..default()
        },
        ..default()
    })
    .insert(Arena)
    .insert(game::GameTag);
}