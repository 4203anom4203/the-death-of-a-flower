use bevy::{
    input_focus::InputFocus, prelude::*
};
use crate::components;
pub fn start_button_system (
    mut input_focus: ResMut<InputFocus>,
    mut state: ResMut<components::GameState>,
    mut interaction_query: Query<(Entity, &Button, &components::StartButton, &mut Visibility, &Interaction), Changed<Interaction>>,
) {
    for (entity, _startbutton, _button, mut visibility, interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity);
                state.state = components::GameStateResource::InGame;
                *visibility = Visibility::Hidden;
                
            }
            
            Interaction::Hovered => {
                input_focus.set(entity);
            }

            Interaction::None => {
                input_focus.clear();
            }
        }
    }
}



pub fn update_title_background (
    mut titlescreen: ResMut<components::TitleScreenState>,
    time: Res<Time>,
    gamestate: Res<components::GameState>,
    asset_server: Res<AssetServer>,
    mut query: Query<&mut ImageNode, With<components::TitleBackgroundImage>>
) {
    titlescreen.timer.tick(time.delta());

    if titlescreen.timer.just_finished() && gamestate.state == components::GameStateResource::StartMenu {
        for mut image_node in &mut query {
            match titlescreen.state {

                components::TitleScreenSwap::Transition1 => {
                    image_node.image = asset_server.load("TitleScreen/Glitch_Frame.png");
                    titlescreen.timer = Timer::from_seconds(0.05, TimerMode::Once);
                    titlescreen.state = components::TitleScreenSwap::BaseLibrary;
                }
                components::TitleScreenSwap::BaseLibrary => {
                    image_node.image = asset_server.load("TitleScreen/Library_Soft.png");
                    titlescreen.timer = Timer::from_seconds(4.95, TimerMode::Once);
                    titlescreen.state = components::TitleScreenSwap::Transition2;
                    
                }

                components::TitleScreenSwap::Transition2 => {
                    image_node.image = asset_server.load("TitleScreen/Glitch_Frame.png");
                    titlescreen.timer = Timer::from_seconds(0.05, TimerMode::Once);
                    titlescreen.state = components::TitleScreenSwap::DecayLibrary;
                }

                components::TitleScreenSwap::DecayLibrary => {
                    image_node.image = asset_server.load("TitleScreen/Decay_Library_Tint.png");
                    titlescreen.timer = Timer::from_seconds(4.95, TimerMode::Once);
                    titlescreen.state = components::TitleScreenSwap::Transition1;
                }

                
            }

        }
    } else if !titlescreen.timer.just_finished() && gamestate.state == components::GameStateResource::InGame {
        for mut image_node in &mut query {
            image_node.image = asset_server.load("blank-background.png");
        }
    }
}//holy shit cursed as fuck logic but it works

pub fn credits_button_system (
    mut input_focus: ResMut<InputFocus>,
    mut state: ResMut<components::CreditsState>,
    mut interaction_query: Query<(Entity, &Interaction, &Button, &components::CreditsButton,), Changed<Interaction>>,
) {
    for (entity, interaction, &Button, &components::CreditsButton) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity);
                state.state = match state.state {
                    components::CreditsResource::Hidden => components::CreditsResource::Shown,
                    components::CreditsResource::Shown => components::CreditsResource::Hidden, //yes very copy paste ikik
                };
            }

            Interaction::Hovered => {
                input_focus.set(entity);
            }

            Interaction::None => {
                input_focus.clear();
            }
        }
    }
}

pub fn update_credits_menu (
    state: Res<components::CreditsState>,
    mut panel_query: Query<&mut Visibility, With<components::CreditsWindow>>,
) {
    for mut visibility in &mut panel_query {
        *visibility = if state.state == components::CreditsResource::Shown {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    
}

pub fn settings_button_system (
    mut input_focus: ResMut<InputFocus>,
    mut state: ResMut<components::MenuState>,
    mut interaction_query: Query<(Entity, &Interaction, /* &mut BorderColor,*/ &Button, &components::SettingsButton), Changed<Interaction>>,
) {
    for (entity, interaction, /*mut border_color,*/ &Button, &components::SettingsButton) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity);
                //*border_color = BorderColor::all(GREEN);
                state.current_menu = match state.current_menu {
                components::Menu::None => components::Menu::Settings,
                components::Menu::Settings => components::Menu::None, //settings overlay toggle
                };
            }

            Interaction::Hovered => {
                input_focus.set(entity);
                //*border_color = BorderColor::all(YELLOW);
            }

            Interaction::None => {
                input_focus.clear();
                //*border_color = BorderColor::all(RED);
            }
        }
    }
}



pub fn update_settings_menu (
    menu: Res<components::MenuState>,
    mut panel_query: Query<&mut Visibility, With<components::SettingsPanel>>,
) {
    for mut visibility in &mut panel_query {
        *visibility = if menu.current_menu == components::Menu::Settings {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    
}

pub fn settings_menu_keybind (
    input: Res<ButtonInput<KeyCode>>,
    mut menu: ResMut<components::MenuState>,
    gamestate: Res<components::GameState>,
) {
    if input.just_pressed(KeyCode::Escape)&&menu.current_menu == components::Menu::Settings {
        menu.current_menu = components::Menu::None;
    } else if input.just_pressed(KeyCode::Escape)&&menu.current_menu == components::Menu::None&&gamestate.state == components::GameStateResource::InGame {
        menu.current_menu = components::Menu::Settings;
    }
}

pub fn button_dissapear (
    state: Res<components::GameState>,
    mut query: Query<&mut Visibility, With<components::MenuButton>>, 
) {
    for mut visibility in &mut query {
        *visibility = if state.state == components::GameStateResource::InGame {
            Visibility::Hidden
        } else {
            Visibility::Visible
        }
    }
}

