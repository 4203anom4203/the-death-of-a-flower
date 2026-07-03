use bevy::prelude::*;
use crate::derive;
pub fn start_button_system (
    mut state: ResMut<derive::GameState>,
    mut interaction_query: Query<(&Button, &derive::StartButton, &mut Visibility, &Interaction), Changed<Interaction>>,
) {
    for (_startbutton, _button, mut visibility, interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                
                state.state = derive::GameStateResource::InGame;
                *visibility = Visibility::Hidden;
                
            }
            
            Interaction::Hovered => {
                
            }

            Interaction::None => {
                
            }
        }
    }
}



pub fn update_title_background (
    mut titlescreen: ResMut<derive::TitleScreenState>,
    time: Res<Time>,
    gamestate: Res<derive::GameState>,
    asset_server: Res<AssetServer>,
    mut query: Query<&mut ImageNode, With<derive::TitleBackgroundImage>>
) {
    titlescreen.timer.tick(time.delta());

    if titlescreen.timer.just_finished() && gamestate.state == derive::GameStateResource::StartMenu {
        for mut image_node in &mut query {
            match titlescreen.state {

                derive::TitleScreenSwap::Transition1 => {
                    image_node.image = asset_server.load("TitleScreen/Glitch_Frame.png");
                    titlescreen.timer = Timer::from_seconds(0.05, TimerMode::Once);
                    titlescreen.state = derive::TitleScreenSwap::BaseLibrary;
                }
                derive::TitleScreenSwap::BaseLibrary => {
                    image_node.image = asset_server.load("TitleScreen/Library_Soft.png");
                    titlescreen.timer = Timer::from_seconds(4.95, TimerMode::Once);
                    titlescreen.state = derive::TitleScreenSwap::Transition2;
                    
                }

                derive::TitleScreenSwap::Transition2 => {
                    image_node.image = asset_server.load("TitleScreen/Glitch_Frame.png");
                    titlescreen.timer = Timer::from_seconds(0.05, TimerMode::Once);
                    titlescreen.state = derive::TitleScreenSwap::DecayLibrary;
                }

                derive::TitleScreenSwap::DecayLibrary => {
                    image_node.image = asset_server.load("TitleScreen/Decay_Library_Tint.png");
                    titlescreen.timer = Timer::from_seconds(4.95, TimerMode::Once);
                    titlescreen.state = derive::TitleScreenSwap::Transition1;
                }

                
            }

        }
    } else if !titlescreen.timer.just_finished() && gamestate.state == derive::GameStateResource::InGame {
        for mut image_node in &mut query {
            image_node.image = asset_server.load("blank-background.png");
        }
    }
}//holy shit cursed as fuck logic but it works

pub fn credits_button_system (
    mut state: ResMut<derive::MenuState>,
    mut interaction_query: Query<(&Interaction, &Button, &derive::CreditsButton,), Changed<Interaction>>,
) {
    for (interaction, &Button, &derive::CreditsButton) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                state.current_menu = match state.current_menu {
                    derive::Menu::None => derive::Menu::Credits,
                    derive::Menu::Credits => derive::Menu::None, //yes very copy paste ikik
                    derive::Menu::Settings => derive::Menu::Settings, //disables settings button in credits menu
                };
            }

            Interaction::Hovered => {
            }

            Interaction::None => {
            }
        }
    }
}

pub fn update_credits_menu (
    state: Res<derive::MenuState>,
    mut panel_query: Query<&mut Visibility, With<derive::CreditsWindow>>,
) {
    for mut visibility in &mut panel_query {
        *visibility = if state.current_menu == derive::Menu::Credits {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    
}

pub fn settings_button_system (
    mut state: ResMut<derive::MenuState>,
    mut interaction_query: Query<(&Interaction, /* &mut BorderColor,*/ &Button, &derive::SettingsButton), Changed<Interaction>>,
) {
    for (interaction, /*mut border_color,*/ &Button, &derive::SettingsButton) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                //*border_color = BorderColor::all(GREEN);
                state.current_menu = match state.current_menu {
                derive::Menu::None => derive::Menu::Settings,
                derive::Menu::Settings => derive::Menu::None, //settings overlay toggle
                derive::Menu::Credits => derive::Menu::Credits, //disable settings button inside of credits menu
                };
            }

            Interaction::Hovered => {
                //*border_color = BorderColor::all(YELLOW);
            }

            Interaction::None => {
                //*border_color = BorderColor::all(RED);
            }
        }
    }
}



pub fn update_settings_menu (
    menu: Res<derive::MenuState>,
    mut panel_query: Query<&mut Visibility, With<derive::SettingsPanel>>,
) {
    for mut visibility in &mut panel_query {
        *visibility = if menu.current_menu == derive::Menu::Settings {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    
}

pub fn exit_menu_keybind (
    input: Res<ButtonInput<KeyCode>>,
    mut menu: ResMut<derive::MenuState>,
    gamestate: Res<derive::GameState>,
) {
    if input.just_pressed(KeyCode::Escape)&& (menu.current_menu == derive::Menu::Settings || menu.current_menu == derive::Menu::Credits) {
        menu.current_menu = derive::Menu::None; //this logic hurts my soul
    } else if input.just_pressed(KeyCode::Escape)&&menu.current_menu == derive::Menu::None&&gamestate.state == derive::GameStateResource::InGame {
        menu.current_menu = derive::Menu::Settings;
    }
}
//handles closing menus with esc, making settings toggle ingame
pub fn button_dissapear (
    state: Res<derive::GameState>,
    mut query: Query<&mut Visibility, With<derive::MenuButton>>, 
) {
    for mut visibility in &mut query {
        *visibility = if state.state == derive::GameStateResource::InGame {
            Visibility::Hidden
        } else {
            Visibility::Visible
        }
    }
}

