use bevy::{ecs::relationship::Relationship, prelude::*, ui_widgets::{SliderThumb, SliderValue}};
use crate::{types, setup};
pub fn start_button_system (
    mut state: ResMut<types::GameState>,
    mut interaction_query: Query<(&Button, &types::StartButton, &mut Visibility, &Interaction), Changed<Interaction>>,
) {
    for (_startbutton, _button, mut visibility, interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                state.state = types::GameStateResource::InGame;
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
    mut titlescreen: ResMut<types::TitleScreenState>,
    time: Res<Time>,
    gamestate: Res<types::GameState>,
    asset_server: Res<AssetServer>,
    mut query: Query<&mut ImageNode, With<types::TitleBackgroundImage>>
) {
    titlescreen.timer.tick(time.delta());

    if titlescreen.timer.just_finished() && gamestate.state == types::GameStateResource::StartMenu {
        for mut image_node in &mut query {
            match titlescreen.state {

                types::TitleScreenSwap::Transition1 => {
                    image_node.image = asset_server.load("TitleScreen/Glitch_Frame.png");
                    titlescreen.timer = Timer::from_seconds(0.05, TimerMode::Once);
                    titlescreen.state = types::TitleScreenSwap::BaseLibrary;
                }
                types::TitleScreenSwap::BaseLibrary => {
                    image_node.image = asset_server.load("TitleScreen/Library_Soft.png");
                    titlescreen.timer = Timer::from_seconds(4.95, TimerMode::Once);
                    titlescreen.state = types::TitleScreenSwap::Transition2;
                    
                }

                types::TitleScreenSwap::Transition2 => {
                    image_node.image = asset_server.load("TitleScreen/Glitch_Frame.png");
                    titlescreen.timer = Timer::from_seconds(0.05, TimerMode::Once);
                    titlescreen.state = types::TitleScreenSwap::DecayLibrary;
                }

                types::TitleScreenSwap::DecayLibrary => {
                    image_node.image = asset_server.load("TitleScreen/Decay_Library_Tint.png");
                    titlescreen.timer = Timer::from_seconds(4.95, TimerMode::Once);
                    titlescreen.state = types::TitleScreenSwap::Transition1;
                }

                
            }

        }
    } else if !titlescreen.timer.just_finished() && gamestate.state == types::GameStateResource::InGame {
        for mut image_node in &mut query {
            image_node.image = asset_server.load("blank-background.png");
        }
    }
}//holy shit cursed as fuck logic but it works

pub fn credits_button_system (
    mut state: ResMut<types::MenuState>,
    mut interaction_query: Query<(&Interaction, &Button, &types::CreditsButton,), Changed<Interaction>>,
) {
    for (interaction, &Button, &types::CreditsButton) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                state.current_menu = match state.current_menu {
                    types::Menu::None => types::Menu::Credits,
                    types::Menu::Credits => types::Menu::None, //yes very copy paste ikik
                    types::Menu::Settings => types::Menu::Settings, //disables settings button in credits menu
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
    state: Res<types::MenuState>,
    mut panel_query: Query<&mut Visibility, With<types::CreditsWindow>>,
) {
    for mut visibility in &mut panel_query {
        *visibility = if state.current_menu == types::Menu::Credits {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    
}

pub fn settings_button_system (
    mut state: ResMut<types::MenuState>,
    mut interaction_query: Query<(&Interaction, /* &mut BorderColor,*/ &Button, &types::SettingsButton), Changed<Interaction>>,
) {
    for (interaction, /*mut border_color,*/ &Button, &types::SettingsButton) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                //*border_color = BorderColor::all(GREEN);
                state.current_menu = match state.current_menu {
                types::Menu::None => types::Menu::Settings,
                types::Menu::Settings => types::Menu::Settings, //settings overlay toggle
                types::Menu::Credits => types::Menu::Credits, //disable settings button inside of credits menu
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
    menu: Res<types::MenuState>,
    mut panel_query: Query<&mut Visibility, With<types::SettingsPanel>>,
) {
    for mut visibility in &mut panel_query {
        *visibility = if menu.current_menu == types::Menu::Settings {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    
}

pub fn exit_menu_keybind (
    input: Res<ButtonInput<KeyCode>>,
    mut menu: ResMut<types::MenuState>,
    gamestate: Res<types::GameState>,
) {
    if input.just_pressed(KeyCode::Escape)&& (menu.current_menu == types::Menu::Settings || menu.current_menu == types::Menu::Credits) {
        menu.current_menu = types::Menu::None; //this logic hurts my soul
    } else if input.just_pressed(KeyCode::Escape)&&menu.current_menu == types::Menu::None&&gamestate.state == types::GameStateResource::InGame {
        menu.current_menu = types::Menu::Settings;
    }
}
//handles closing menus with esc, making settings toggle ingame
pub fn button_dissapear (
    state: Res<types::GameState>,
    mut query: Query<&mut Visibility, With<types::MenuButton>>, 
) {
    for mut visibility in &mut query {
        *visibility = if state.state == types::GameStateResource::InGame {
            Visibility::Hidden
        } else {
            Visibility::Visible
        }
    }
}
//slider internals already handled
pub fn update_sliders (
    thumb: Query<(&mut Node, &ChildOf), With<SliderThumb>>,
    track: Query<&ChildOf, Without<SliderThumb>>,
    slider: Query<&SliderValue , Changed<SliderValue>>,
) {
    for (mut thumb_node, thumb_parent) in thumb {
        if let Ok(track_parent) = track.get(thumb_parent.get()) { //from my understanding, this checks if getting the thumb parent is a success, then sets the track parent to that value then it checks if it went through
            if let Ok(slider_value) = slider.get(track_parent.get()) { //depends on the query, this shit is hard
                let value = (slider_value.0 - setup::SLIDER_MIN) / (setup::SLIDER_MAX - setup::SLIDER_MIN);
                let percent = value.clamp(setup::SLIDER_MIN, setup::SLIDER_MAX) * 100.0;
                thumb_node.left = Val::Percent(percent);
            }
        }
        
    }
}

pub fn update_volume (
    slider: Query<(&SliderValue, &types::AudioSettingsComponent), Changed<SliderValue>>,
    mut audio: ResMut<types::AudioSettings>,
) {
        for (slider_value, binding) in &slider {
            match binding {
                types::AudioSettingsComponent::Voice => {
                    audio.voice_volume = (slider_value.0 - setup::SLIDER_MIN) / (setup::SLIDER_MAX - setup::SLIDER_MIN) * 100.0;
                }
                types::AudioSettingsComponent::Sfx => {
                    audio.sfx_volume = (slider_value.0 - setup::SLIDER_MIN) / (setup::SLIDER_MAX - setup::SLIDER_MIN) * 100.0;
                }
                types::AudioSettingsComponent::Music => {
                    audio.music_volume = (slider_value.0 - setup::SLIDER_MIN) / (setup::SLIDER_MAX - setup::SLIDER_MIN) * 100.0;
                }

            }
        }
}
//updates internal variables or somth
pub fn update_volume_numbers (
    mut number: Query<(&mut Text, &types::AudioSettingsComponent), With<types::SettingsTextNode>>,
    audio: Res<types::AudioSettings>,
) {
    for (mut text, binding) in &mut number {
        match binding {
            types::AudioSettingsComponent::Voice => {
                text.0 = format!("Voice Volume: {}%", audio.voice_volume);
            }
            types::AudioSettingsComponent::Sfx => {

            }
            types::AudioSettingsComponent::Music => {

            }
            
        }
    }
}