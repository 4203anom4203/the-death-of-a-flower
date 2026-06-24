use bevy::{
    input_focus::InputFocus, prelude::*
};
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
mod setup;
mod derive;
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
        .init_resource::<derive::MenuState>()
        .init_resource::<derive::GameState>()
        .init_resource::<derive::CreditsState>()
        .add_systems(Startup, setup::setup)
        .add_systems(Update, systems::settings_button_system)
        .add_systems(Update, systems::update_settings_menu)
        .add_systems(Update, systems::update_title_background)
        .add_systems(Update, systems::settings_menu_keybind)
        .add_systems(Update, systems::start_button_system)
        .add_systems(Update, systems::button_dissapear)
        .add_systems(Update, systems::credits_button_system)
        .add_systems(Update, systems::update_credits_menu)
        .run()
}



