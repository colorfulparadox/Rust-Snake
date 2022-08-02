use bevy::prelude::*;
use bevy::core::FixedTimestep;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::*;

pub mod snake;
pub mod food;
mod highscore;
mod arena;
mod death_screen;

#[derive(Component)]
pub struct GameTag;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {

        app
            .add_event::<snake::AddToSnakeBody>()
            .add_event::<snake::KillSnake>()
            .add_event::<highscore::HighScoreEvent>()

            .add_system_set(SystemSet::on_enter(AppState::Game)
                .with_system(arena::setup)
                .with_system(highscore::setup)
                .with_system(food::spawn)
                .with_system(snake::setup)
            )
                
            .add_system_set(SystemSet::on_update(AppState::Game)
                .with_system(snake::input)
                .with_system(snake::kill_update)
                .with_system(snake::kill_snake)
                .with_system(snake::add_to_snake)
                .with_system(food::eat_food)
                .with_system(highscore::highscore_event)
            )
            
            .add_system_set(SystemSet::on_enter(GameState::InGame)
                .with_system(snake::spawn)
                .with_system(despawn_screen::<game::death_screen::DeathScreenTag>)
            )

            .add_system_set(SystemSet::on_enter(GameState::Dead)
                .with_system(death_screen::setup)
            )

            .add_system_set(SystemSet::on_update(GameState::Dead)
                .with_system(death_screen::button_effects)
                .with_system(death_screen::button_update)
            )

            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.1))
                    .with_system(snake::update)
            );

    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup(

) {
    
}

fn update() {
    
}