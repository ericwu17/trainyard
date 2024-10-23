use bevy::prelude::*;

#[derive(Component)]
pub struct TrainyardButton;

// util function for creating a button and getting a handle to the entity
pub fn create_trainyard_button(
    commands: &mut Commands,
    text: &str,
    width_px: f32,
    height_px: f32,
    text_size: f32,
    border_color: Color,
    font: Handle<Font>,
) -> Entity {
    let button_bundle = (
        ButtonBundle {
            style: Style {
                width: Val::Px(width_px),
                height: Val::Px(height_px),
                border: UiRect::all(Val::Px(3.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            border_color: border_color.into(),
            border_radius: BorderRadius::all(Val::Px(12.0)),
            background_color: super::BTN_BG.into(),
            ..default()
        },
        TrainyardButton,
    );

    let text_bundle = TextBundle::from_section(
        text,
        TextStyle {
            font,
            font_size: text_size,
            color: Color::srgb(1.0, 1.0, 1.0),
            ..default()
        },
    )
    .with_text_justify(JustifyText::Center)
    .with_style(Style {
        position_type: PositionType::Absolute,
        width: Val::Percent(100.0),
        ..default()
    });

    let button_entity = commands.spawn(button_bundle).id();
    let text_entity = commands.spawn(text_bundle).id();
    commands.entity(button_entity).push_children(&[text_entity]);
    return button_entity;
}

pub fn button_sounds_system(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<TrainyardButton>)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            commands.spawn(AudioBundle {
                source: asset_server.load("audio/button_press.ogg"),
                ..default()
            });
        }
    }
}
