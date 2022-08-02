use bevy::{prelude::*, sprite::Anchor, asset};
use rand::Rng;

use crate::*;
use crate::game::*;

#[derive(Component)]
pub struct Food;

pub fn spawn(
    mut commands: Commands, 
    textures: Res<resources::GameTextures>,
    settings: Res<Settings>, 
) {
    let width = (arena::ARENA_SIZE as i32) - 16;
    let height = (arena::ARENA_SIZE as i32);;

    let x: i32 = 16 * rand::thread_rng().gen_range(1..=(width/16));
    let y: i32 = 16 * rand::thread_rng().gen_range(1..=(height/16));

    let sprite_sheet_bundle: SpriteSheetBundle = SpriteSheetBundle {
        texture_atlas: textures.snake_atlas.clone(),
        sprite: TextureAtlasSprite {
            index: 2,
            anchor: Anchor::TopLeft,
            ..default()
        },
        transform: Transform {
            translation: new_food_pos(),
            ..default()
        },
        ..default()
    };

    //println!("food pos: x:{x} y:{y}");

    commands.spawn()
        .insert(Food)
        .insert(game::GameTag)
        .insert_bundle(sprite_sheet_bundle);


}

/*
pub fn eat_food(
    mut commands: Commands,
    mut set: ParamSet<(Query<(&mut Transform), With<Food>>, Query<(&Transform, &mut snake::SnakeHead)>)>,
    mut snake_event: EventWriter<snake::AddToSnakeBody>,
) {

    if set.p0().is_empty() || set.p1().is_empty() {
        return;
    }

    let mut same_pos = false;
    {
        let mut snake_query = set.p1();
        let (snake_transform, mut snake_head) = snake_query.single_mut();
        let snake_pos: Vec2 = Vec2::new(snake_transform.translation.x, snake_transform.translation.y);

        let mut food_query = set.p0();
        let mut food_transform = food_query.single_mut();
        let food_pos: Vec2 = Vec2::new(food_transform.translation.x, food_transform.translation.y);
        
        if food_pos.eq(&snake_pos) {
            same_pos = true;
            food_transform.translation = new_food_pos();
        } 
    }

    if same_pos {
        snake_event.send(snake::AddToSnakeBody);

        let mut snake_query = set.p1();
        let (snake_transform, mut snake_head) = snake_query.single_mut();
        let snake_pos: Vec2 = Vec2::new(snake_transform.translation.x, snake_transform.translation.y);

        snake_head.score += 1;
        snake_event.send(snake::AddToSnakeBody);
    }

}
*/

pub fn eat_food(
    mut commands: Commands,
    mut set: ParamSet<(Query<(&mut Transform), With<Food>>, Query<(&Transform, &mut snake::SnakeHead)>)>,
    mut snake_event: EventWriter<snake::AddToSnakeBody>,
) {
    if set.p0().is_empty() || set.p1().is_empty() {
        return;
    }

    let mut snake_query = set.p1();
    let (snake_transform, _) = snake_query.single();
    let snake_pos = snake_transform.translation.truncate();

    let mut food_query = set.p0();
    let mut food_transform = food_query.single_mut();
    let food_pos = food_transform.translation.truncate();

    if food_pos.eq(&snake_pos) {       
        food_transform.translation = new_food_pos();
        snake_event.send(snake::AddToSnakeBody);

        let mut snake_query = set.p1();
        let (_, mut snake_head) = snake_query.single_mut();
        snake_head.score += 1;
    }
}

fn new_food_pos() -> Vec3 {
    let width = (arena::ARENA_SIZE as i32) - 16;
    let height = (arena::ARENA_SIZE as i32);;

    let x: i32 = 16 * rand::thread_rng().gen_range(1..=(width/16));
    let y: i32 = 16 * rand::thread_rng().gen_range(1..=(height/16));

    Vec3::new(x as f32, y as f32, 0.)
}