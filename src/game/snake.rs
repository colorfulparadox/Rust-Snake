use bevy::{prelude::*, sprite::Anchor};

use iyes_loopless::prelude::*;

use crate::*;
use crate::game::*;

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right 
}

#[derive(Component)]
pub struct SnakeHead {
    direction: Direction,
    pub score: i32
}

#[derive(Component)]
pub struct SnakeBody;

//Events
pub struct AddToSnakeBody;
pub struct KillSnake;

pub fn setup(
    mut game_state: ResMut<State<GameState>>,
) {
    game_state.set(GameState::InGame);
}

pub fn spawn(
    mut commands: Commands,
    textures: Res<resources::GameTextures>,
    mut snake_body_event: EventWriter<AddToSnakeBody>,
) {
    let sprite_sheet_bundle: SpriteSheetBundle = SpriteSheetBundle {
        texture_atlas: textures.snake_atlas.clone(),
        sprite: TextureAtlasSprite {
            index: 0,
            anchor: Anchor::TopLeft,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(240., 240., 0.),
            ..default()
        },
        ..default()
    };

    commands.spawn()
        .insert(SnakeHead {
            direction: Direction::Left,
            score: 0
        } )
        .insert(game::GameTag)
        .insert_bundle(sprite_sheet_bundle);

    snake_body_event.send(snake::AddToSnakeBody);
}

pub fn update(
    mut commands: Commands,
    mut app_state: Res<State<AppState>>,
    mut set: ParamSet<(Query<(&mut Transform, &mut SnakeHead)>, Query<(&mut Transform, With<SnakeBody>)>)>,
) {
    //check if state is in game a hacky way to do this because bevy is dumb dumb sometimes
    let state: &AppState = app_state.current();
    if state.eq(&AppState::Menu) { return }

    //lets do our head movement update
    let mut head_query = set.p0();
    if head_query.is_empty() { return }
    let (mut transform, mut snake_head) = head_query.single_mut();
    let direction: Vec2 = Vec2::new(16.,16.) * get_velo(snake_head.direction);
    
    let last_pos: Vec3 = transform.translation.clone();
    transform.translation = transform.translation + Vec3::new(direction.x, direction.y, 0.);

    //now lets update our body
    let mut body_query = set.p1();
    let mut pos = last_pos;
    for (mut transform, bool) in body_query.iter_mut() {
        let old_trans = transform.translation.clone();

        transform.translation = pos;

        pos = old_trans;
    }
}

pub fn input(
    input: Res<Input<KeyCode>>, 
    mut snake_query: Query<(&mut SnakeHead)>,
) {
    if snake_query.is_empty() {
        return;
    }

    let mut snake = snake_query.single_mut();

    if input.pressed(KeyCode::Up) || input.pressed(KeyCode::W){
        snake.direction = Direction::Up;
    } else if input.pressed(KeyCode::Down) || input.pressed(KeyCode::S) {
        snake.direction = Direction::Down;
    } else if input.pressed(KeyCode::Left) || input.pressed(KeyCode::A) {
        snake.direction = Direction::Left;
    } else if input.pressed(KeyCode::Right) || input.pressed(KeyCode::D) {
        snake.direction = Direction::Right;
    }
}

pub fn kill_update(
    mut commands: Commands,
    mut set: ParamSet<(Query<(Entity, &Transform, With<SnakeHead>)>, Query<(Entity, &Transform, With<SnakeBody>)>)>,
    mut kill_snake: EventWriter<snake::KillSnake>,
) {
    if set.p0().is_empty() || set.p1().is_empty() {
        return;
    }
    
    let mut head_query = set.p0();
    let (head_entity, head_transform, bool) = head_query.single();
    let head_pos: Vec3 = head_transform.translation;

    let mut body_query = set.p1();


    //out of bounds check
    if head_pos.y > 480. || head_pos.y <= 0. {
        kill_snake.send(KillSnake);
        return;
    }

    if head_pos.x >= 480. || head_pos.x <= 0. {
        kill_snake.send(KillSnake);
        return;
    }

    //colliding with body check
    for (body_entity, body_transform, bool) in body_query.iter() {
        if body_transform.translation.eq(&head_pos) {
            kill_snake.send(KillSnake);
            return;
        } 
    }

}

pub fn kill_snake(
    mut commands: Commands,
    head_query: Query<(Entity, &SnakeHead)>,
    body_query: Query<(Entity, With<SnakeBody>)>,
    mut event: EventReader<KillSnake>,
    mut highscore_event: EventWriter<highscore::HighScoreEvent>,
    mut game_state: ResMut<State<GameState>>,
) {
    if body_query.is_empty() { return }

    for ev in event.iter() {
        game_state.set(GameState::Dead);

        for (head_entity, snake_head) in head_query.iter() {
            highscore_event.send(highscore::HighScoreEvent(snake_head.score));
            commands.entity(head_entity).despawn();
        }

        for (body_entity, bool) in body_query.iter() {
            commands.entity(body_entity).despawn();
        }
    }
}

pub fn add_to_snake(
    mut commands: Commands,
    mut event: EventReader<AddToSnakeBody>,
    textures: Res<resources::GameTextures>, 
) {
    for ev in event.iter() {
        let sprite_sheet_bundle: SpriteSheetBundle = SpriteSheetBundle {
            texture_atlas: textures.snake_atlas.clone(),
            sprite: TextureAtlasSprite {
                index: 1,
                anchor: Anchor::TopLeft,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(2500.,2500., 0.),
                ..default()
            },
            ..default()
        };
        
        commands.spawn()
            .insert(SnakeBody)
            .insert_bundle(sprite_sheet_bundle);
    }
}

fn get_velo(dir: Direction) -> Vec2 {
    match dir {
        Direction::Up => return Vec2::new(0., 1.),
        Direction::Down => return Vec2::new(0., -1.),
        Direction::Left => return Vec2::new(-1., 0.),
        Direction::Right => return Vec2::new(1., 0.),
    }
}