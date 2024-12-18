pub mod buttons;
pub mod credits;
pub mod level;
pub mod level_picker;
pub mod main_menu;

use bevy::prelude::*;

#[derive(Component)]
pub struct UIRootContainer;

// button background color
pub const BTN_BG: Color = Color::srgb(0.15, 0.15, 0.15);
pub const BTN_BORDER_GREEN: Color = Color::srgb(0.365, 0.573, 0.329);
pub const BTN_BORDER_BLUE: Color = Color::srgb(0.329, 0.412, 0.572);
pub const BTN_BORDER_BLACK: Color = Color::BLACK;

#[derive(States, Default, Debug, PartialEq, Eq, Hash, Clone)]
pub enum UIState {
    /// The None state is used briefly at the time the game loads, so that we can transition the state into the
    /// "true initial state" of MainMenu __after__ we spawn the UIRootContainer.
    #[default]
    None,
    MainMenu,
    LevelPicker,
    Level,
    Credits,
}

pub struct TrainyardUIPlugin;

impl Plugin for TrainyardUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<UIState>()
            .add_plugins((
                main_menu::MainMenuUIPlugin,
                level_picker::LevelPickerUIPlugin,
                level::LevelUIPlugin,
                credits::CreditsUIPlugin,
                buttons::ButtonPlugin,
            ))
            .add_systems(
                Startup,
                (spawn_ui_root_container, set_initial_ui_state).chain(),
            );
    }
}

fn spawn_ui_root_container(mut commands: Commands) {
    let root_container = (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        UIRootContainer,
    );
    commands.spawn(root_container);
}

fn set_initial_ui_state(mut next_state: ResMut<NextState<UIState>>) {
    next_state.set(UIState::MainMenu);
}
