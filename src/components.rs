use bevy::{
    prelude::*
};

#[derive(Default, PartialEq)]
pub enum Menu {
    #[default]
    None,
    Settings,
    //can add shit
}
//resources
#[derive(Resource, Default)]
pub struct MenuState {
    pub current_menu: Menu,
}

//components
#[derive(Component)]
pub struct SettingsButton;

#[derive(Component)]
pub struct SettingsPanel;

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

#[derive(Component)]
pub struct WorldCamera;

#[derive(Component)]
pub struct TitleBackgroundImage;

#[derive(Component)]
pub struct StartButton;

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

#[derive(Component)]
pub struct CreditsButton;

#[derive(Resource, Default)]
pub struct CreditsState {
    pub state: CreditsResource,
}

#[derive(Resource, Default, PartialEq)]
pub enum CreditsResource {
    #[default]
    Hidden,
    Shown,
}

#[derive(Component)]
pub struct CreditsWindow;

#[derive(Component)]
pub struct MenuButton;
//my linter blew up because of unused code