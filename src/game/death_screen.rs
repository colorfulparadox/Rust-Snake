use bevy::prelude::*;

use crate::*;

#[derive(Component)]
pub enum ButtonAction {
    Replay,
    Menu
}

#[derive(Component)]
pub struct DeathScreenTag;

pub fn setup(
    mut commands: Commands,
    font_assets: Res<resources::GameFonts>,
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
        font_size: 20.0,
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
    .insert(DeathScreenTag)
    .insert(game::GameTag)
    .id();
    
    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: "GAME OVER".to_string(),
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
                    value: "REPLAY".to_string(),
                    style: button_txt_style.clone(),
                }],
                ..Default::default()
            },
            ..Default::default()
        });
    })
    .insert(Parent(container))
    .insert(ButtonAction::Replay);

    commands.spawn_bundle(ButtonBundle {
        style: button_style.clone(),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "MENU".to_string(),
                    style: button_txt_style.clone(),
                }],
                ..Default::default()
            },
            ..Default::default()
        });
    })
    .insert(Parent(container))
    .insert(ButtonAction::Menu);

}

pub fn button_effects(
    mut commands: Commands,
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

pub fn button_update(
    interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
    mut game_state: ResMut<State<GameState>>,
    mut app_state: ResMut<State<AppState>>,
) {
    let mut replay_action = || {
        game_state.set(GameState::InGame);
    };

    let mut menu_action = || {
        app_state.set(AppState::Menu);
    };

    for (interaction, menu_button_action) in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            match menu_button_action {
                ButtonAction::Replay => replay_action(),
                ButtonAction::Menu => menu_action()
            }
        }
    }
}