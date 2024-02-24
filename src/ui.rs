use crate::{game_assets::UiFont, state::GameGlobals, FONT_SIZE, UI_HEIGHT, WINDOW_HEIGHT};
use bevy::{prelude::*, window::WindowResized};

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, load_ui);
        app.add_systems(Update, update_score);
        app.add_systems(Update, resize_ui.run_if(on_event::<WindowResized>()));
    }
}

#[derive(Component)]
struct ScoreText;

fn load_ui(mut commands: Commands, font: Res<UiFont>) {
    let ui_container = NodeBundle {
        style: Style {
            // fill the entire with of the window window
            width: Val::Percent(100.),
            height: Val::Px(UI_HEIGHT),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        background_color: BackgroundColor(Color::BLACK),
        ..Default::default()
    };

    let row = NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            ..Default::default()
        },
        ..Default::default()
    };

    let hiscore_text = TextBundle::from_section(
        "HIGH SCORE",
        TextStyle {
            font: font.default.clone(),
            font_size: FONT_SIZE,
            color: Color::WHITE,
        },
    );

    let score_text = TextBundle::from_section(
        String::new(),
        TextStyle {
            font: font.default.clone(),
            font_size: FONT_SIZE,
            color: Color::WHITE,
        },
    );

    commands
        .spawn(ui_container)
        .with_children(|builder| {
            builder.spawn(row.clone()).with_children(|builder| {
                builder.spawn(hiscore_text);
            });
        })
        .with_children(|builder| {
            builder.spawn(row).with_children(|builder| {
                builder.spawn((score_text, ScoreText));
            });
        });
}

fn update_score(mut query: Query<&mut Text, With<ScoreText>>, game_state: Res<GameGlobals>) {
    let mut text = query.single_mut();
    text.sections[0].value = game_state.score.to_string();
}

fn resize_ui(mut ui_scale: ResMut<UiScale>, mut events: EventReader<WindowResized>) {
    let event = events.read().next().unwrap();
    ui_scale.0 = event.height / (WINDOW_HEIGHT);
}
