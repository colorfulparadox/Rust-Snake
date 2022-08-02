use bevy::prelude::*;
use bevy::app::AppExit;

use crate::*;
use crate::resources::*;

mod components;

#[derive(Component)]
enum MenuButtonAction {
    Play,
    Quit
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::Menu)
                .with_system(despawn_screen::<game::GameTag>)
                .with_system(setup))
                
            .add_system_set(SystemSet::on_update(AppState::Menu)
                .with_system(update)
                .with_system(button_effects)
                .with_system(button_update))
            .add_system_set(SystemSet::on_exit(AppState::Menu)
                        .with_system(despawn_screen::<components::MenuTag>));
    }
}

fn setup(
    mut commands: Commands,
    font_assets: Res<resources::GameFonts>,
    player_data: Res<PlayerData>
) { 
    let button_style: Style = Style {
        size: Size::new(Val::Px(120.0), Val::Px(50.0)),
        margin: Rect::all(Val::Px(8.)),
        padding: Rect::all(Val::Px(8.)),
        align_content: AlignContent::Center,
        align_items: AlignItems::Center,
        align_self: AlignSelf::Center,
        justify_content: JustifyContent::Center,
        ..Default::default()
    };

    let button_txt_style : TextStyle = TextStyle {
        font: font_assets.bold.clone(),
        font_size: 24.0,
        color: Color::rgb(0.2, 0.2, 0.9),
    };

    let title_txt_style : TextStyle = TextStyle {
        font: font_assets.bold.clone(),
        font_size: 40.0,
        color: Color::rgb(1., 1., 1.),
    };

    let container = commands
    .spawn_bundle(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect::all(Val::Px(0.)),
            margin: Rect::all(Val::Px(16.)),
            padding: Rect::all(Val::Px(16.)),
            flex_direction: FlexDirection::ColumnReverse,
            align_content: AlignContent::Center,
            align_items: AlignItems::Center,
            align_self: AlignSelf::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        color: UiColor(Color::NONE),
        ..Default::default()
    })
    .insert(components::MenuTag)
    .id();
    
    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: "SNAKE".to_string(),
                style: title_txt_style.clone(),
            }],
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Parent(container));

    commands.spawn_bundle(ButtonBundle {
        style: button_style.clone(),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "Play".to_string(),
                    style: button_txt_style.clone(),
                }],
                ..Default::default()
            },
            ..Default::default()
        });
    })
    .insert(Parent(container))
    .insert(MenuButtonAction::Play);

    commands.spawn_bundle(ButtonBundle {
        style: button_style.clone(),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "Quit".to_string(),
                    style: button_txt_style.clone(),
                }],
                ..Default::default()
            },
            ..Default::default()
        });
    })
    .insert(Parent(container))
    .insert(MenuButtonAction::Quit);

}

fn update() {
    
}

fn button_effects(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    mut interaction_query: Query<(Entity, &Interaction, &mut UiColor, &Children), With<Button>>,
) {
    for (button, interaction, mut color, children) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = Color::rgb(0.5, 0.5, 0.5).into();
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.8, 0.8, 0.8).into();
            }
            Interaction::None => {
                *color = Color::rgb(1., 1., 1.).into();
            }
        }
    }
}

fn button_update(
    interaction_query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_exit_event: EventWriter<AppExit>,
    mut app_state: ResMut<State<AppState>>,
) {
    let mut quit_button = || {
        app_exit_event.send(AppExit);
    };

    let mut play_button = || {
        app_state.set(AppState::Game).unwrap();
    };

    for (interaction, menu_button_action) in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            match menu_button_action {
                MenuButtonAction::Quit => quit_button(),
                MenuButtonAction::Play => play_button()
            }
        }
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}