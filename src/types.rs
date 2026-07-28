use bevy::{
    prelude::*
};

#[derive(Default, PartialEq)]
pub enum Menu {
    #[default]
    None,
    Settings,
    Credits,
    //can add shit
}

#[derive(Resource, Default)]
pub struct MenuState {
    pub current_menu: Menu,
}


#[derive(Resource, Default)]
pub enum TitleScreenSwap {
    #[default]
    BaseLibrary,
    DecayLibrary,
    Transition1,
    Transition2,
}

#[derive(Resource, Default)]
pub struct TitleScreenState {
    pub state: TitleScreenSwap,
    pub timer: Timer,
}

#[derive(Resource, Default, PartialEq)]
pub enum GameStateResource {
    #[default]
    StartMenu,
    InGame,
}

#[derive(Resource, Default)]
pub struct GameState {
    pub state: GameStateResource
}

#[derive(Resource, Default)]
pub struct AudioSettings {
    pub voice_volume: f32,
    pub sfx_volume: f32,
    pub music_volume: f32,
}

#[derive(Component, Debug, Clone, Copy)]
pub enum AudioSettingsComponent {
    Voice,
    Sfx,
    Music,
}
//components

#[derive(Component)]
pub struct CreditsButton;

#[derive(Component)]
pub struct SettingsButton;

#[derive(Component)]
pub struct SettingsPanel;

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct CreditsWindow;

#[derive(Component)]
pub struct MenuButton;

#[derive(Component)]
pub struct WorldCamera;

#[derive(Component)]
pub struct TitleBackgroundImage;
//my linter blew up because of unused code