use rand::prelude::*;

use crate::game::*;

#[derive(Component)]
pub struct SnakeBodyText;

//events
pub struct HighScoreEvent(pub i32);

pub fn setup(
    mut commands: Commands,
    font_assets: Res<resources::GameFonts>,
    player_data: Res<game::PlayerData>
) {
    let title_txt_style : TextStyle = TextStyle {
        font: font_assets.bold.clone(),
        font_size: 12.0,
        color: Color::rgb(1., 1., 1.),
    };

    let mut hs_string: String = "HIGHSCORE: ".to_string();
    let num_string: String = player_data.highscore.to_string();
    hs_string.push_str(num_string.as_str());

    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: hs_string,
                style: title_txt_style.clone(),
            }],
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(game::GameTag)
    .insert(SnakeBodyText);
}

pub fn highscore_event(
    mut highscore_event: EventReader<HighScoreEvent>,
    mut query: Query<(&mut Text), With<SnakeBodyText>>,
    mut player_data: ResMut<game::PlayerData>
) {
    if query.is_empty() { return }

    let mut text = query.single_mut();

    for event in highscore_event.iter() {
        let score: i32 = event.0;
  
        if player_data.highscore >= score { return }

        player_data.highscore = score;

        let mut hs_string: String = "HIGHSCORE: ".to_string();
        let num_string: String = player_data.highscore.to_string();
        hs_string.push_str(num_string.as_str());

        text.sections[0].value = hs_string;
    }
    
}