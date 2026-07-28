use bevy::{
    input_focus::InputFocus, prelude::*,
};
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
mod setup;
mod types;
mod systems;
fn main() -> AppExit {
    App::new()
        .add_plugins((
            EmbeddedAssetPlugin {
                mode: PluginMode::ReplaceDefault,
            },
            DefaultPlugins,
        ))
        .init_resource::<InputFocus>()
        .init_resource::<types::MenuState>()
        .init_resource::<types::GameState>()
        .init_resource::<types::AudioSettings>()
        .add_systems(Startup, setup::setup)
        .add_systems(Update, systems::settings_button_system)
        .add_systems(Update, systems::update_settings_menu)
        .add_systems(Update, systems::update_title_background)
        .add_systems(Update, systems::exit_menu_keybind)
        .add_systems(Update, systems::start_button_system)
        .add_systems(Update, systems::button_dissapear)
        .add_systems(Update, systems::credits_button_system)
        .add_systems(Update, systems::update_credits_menu)
        .add_systems(Update, systems::update_sliders)
        .add_systems(Update, systems::update_volume)
        .run()
}

//whenever i set a system to update, it will run each frame, thats going to fuck over some lower end computers, so i shall make as little as possible, theres like 7 alreadly :(

