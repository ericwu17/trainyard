use bevy::{prelude::*, ui::RelativeCursorPosition};

use crate::ui::{UIState, BTN_BG, BTN_BORDER_BLUE};

pub const SLIDER_BUTTON_WIDTH: f32 = 50.0;
pub const DEFAULT_TRAIN_SPEED: f32 = 0.3;

#[derive(Component)]
pub struct SpeedSliderButton;

#[derive(Component)]
pub struct SpeedSliderSpacer;

#[derive(Event, Debug)]
pub struct ChangeSpeedEvent {
    pub delta: f32,
}

#[derive(Resource)]
pub struct TrainSpeed(pub f32);

pub struct SpeedSliderPlugin;
impl Plugin for SpeedSliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ChangeSpeedEvent>()
            .insert_resource::<TrainSpeed>(TrainSpeed(DEFAULT_TRAIN_SPEED))
            .add_systems(
                Update,
                (
                    handle_speed_slider_interactions,
                    handle_change_speed_events,
                    update_speed_slider_position.run_if(resource_changed::<TrainSpeed>),
                )
                    .chain()
                    .run_if(in_state(UIState::Level)),
            );
    }
}

pub fn spawn_speed_slider(commands: &mut Commands, font: Handle<Font>) -> Entity {
    let slider_button_width = SLIDER_BUTTON_WIDTH;
    let slider_width = super::BUTTON_WIDTH;
    let slider_height = 30.0;
    let slider_text_size = 19.0;

    let slider_background = NodeBundle {
        style: Style {
            width: Val::Px(slider_width),
            height: Val::Px(slider_height),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        border_color: Color::WHITE.into(),
        border_radius: BorderRadius::all(Val::Px(12.0)),
        background_color: Color::BLACK.into(),
        ..default()
    };

    // =============================================================================================
    // spacer (to control slider's horizontal position)
    // =============================================================================================
    let slider_spacer = (
        NodeBundle {
            style: Style {
                width: Val::Px(0.0),
                height: Val::Percent(100.0),
                ..default()
            },
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.0).into(),
            ..default()
        },
        SpeedSliderSpacer,
    );

    // =============================================================================================
    // button_bundle bundle (for the part of the slider that moves)
    // =============================================================================================
    let button_bundle = (
        ButtonBundle {
            style: Style {
                width: Val::Px(slider_button_width),
                height: Val::Px(slider_height),
                border: UiRect::all(Val::Px(3.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            border_color: BTN_BORDER_BLUE.into(),
            border_radius: BorderRadius::all(Val::Px(12.0)),
            background_color: BTN_BG.into(),
            ..default()
        },
        RelativeCursorPosition::default(),
        SpeedSliderButton,
    );
    // =============================================================================================
    // slider text "SPEED"
    // =============================================================================================
    let text_component = TextBundle::from_section(
        "SPEED",
        TextStyle {
            font,
            font_size: slider_text_size,
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

    let slider_background = commands.spawn(slider_background).id();
    let slider_spacer = commands.spawn(slider_spacer).id();
    let button_bundle = commands.spawn(button_bundle).id();
    let text_component = commands.spawn(text_component).id();

    commands
        .entity(slider_background)
        .push_children(&[slider_spacer, button_bundle]);
    commands
        .entity(button_bundle)
        .push_children(&[text_component]);

    slider_background
}

fn handle_speed_slider_interactions(
    interaction_query: Query<(&Interaction, &RelativeCursorPosition), With<SpeedSliderButton>>,
    mut train_speed_event_writer: EventWriter<ChangeSpeedEvent>,
) {
    for (interaction, rel_position) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            if let Some(rel_position) = rel_position.normalized {
                let sensitivity = 0.3; // the sensitivity was determined by trial and error
                let delta = (rel_position.x - 0.5) * sensitivity;
                train_speed_event_writer.send(ChangeSpeedEvent { delta });
            }
        }
    }
}

fn handle_change_speed_events(
    mut train_speed_event_reader: EventReader<ChangeSpeedEvent>,
    mut speed: ResMut<TrainSpeed>,
) {
    for event in train_speed_event_reader.read() {
        let speed = &mut speed.0;
        *speed += event.delta;
        if *speed < 0.0 {
            *speed = 0.0;
        }
        if *speed > 1.0 {
            *speed = 1.0;
        }
    }
}

fn update_speed_slider_position(
    speed: Res<TrainSpeed>,
    mut spacer_query: Query<&mut Style, With<SpeedSliderSpacer>>,
) {
    let slider_border_size = 3.0;

    let new_spacer_size =
        speed.0 * (super::BUTTON_WIDTH - SLIDER_BUTTON_WIDTH - 2.0 * slider_border_size);

    for mut spacer_style in spacer_query.iter_mut() {
        spacer_style.width = Val::Px(new_spacer_size);
    }
}
