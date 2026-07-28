
use bevy::{
    color::palettes::css::WHITE, input_focus::AutoFocus, prelude::*, sprite::Anchor, ui_widgets::{Slider, SliderOrientation::Horizontal, SliderPrecision, SliderRange, SliderThumb, SliderValue, TrackClick::Snap, observe, slider_self_update,}, window::{Window, WindowMode},
};
use crate::types::{self, AudioSettings};

//setup func is for setting up title screen, everything else can move after

pub const PURPLE: Color = Color::srgba(0.749, 0.0, 1.0, 1.0);
pub const SLIDER_MIN: f32 = 0.0;
pub const SLIDER_MAX: f32 = 1.0;
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut window: Single<&mut Window>,
    audio_settings: Res<AudioSettings>,
) {
    //higher render layer = on top :)
    //TODO: Make the background go blank with the thingy
    /////// if statement on the titlescreenswap.
    //TODO: settings buttons in menup
    //TODO: SAVE FILES

    window.mode = WindowMode::BorderlessFullscreen(
        MonitorSelection::Primary,
    );

    commands.spawn((
        //camera
        Camera2d,
        types::WorldCamera,
        Camera {
            //This camera is for everything now
            order: 0,
            clear_color: ClearColorConfig::Custom(Color::Srgba(WHITE)),
            //cursed as hell syntax
            ..default()
        },
    ));

    //main node to render on
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::FlexStart,
            align_content: AlignContent::FlexStart,
            ..default()
        }, //root node for everything

        children![
            (
                types::TitleBackgroundImage,
                BackgroundColor(Color::WHITE),
                ZIndex(0), //all background images will spawn on 0
                ImageNode {
                    image: asset_server.load("TitleScreen/Library_Soft.png"),
                    image_mode: NodeImageMode::Auto,
                    ..default()
                },
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default() //bg
                },
            ),

            (
                Button,
                AutoFocus,
                types::SettingsButton,
                types::MenuButton,
                ZIndex(5), //simple ui button, but the sprites will render on 3 or something
                Visibility::Visible,
                Node {
                    position_type: PositionType::Absolute,
                    height: Val::Percent(10.0),
                    width: Val::Percent(17.5),
                    left: Val::Percent(10.0),
                    top: Val::Percent(30.0),
                    ..default()
                },

                children![(
                    Text::new("Settings"),
                    TextFont {
                        font: bevy::prelude::FontSource::Handle(asset_server.load("fonts/NotoSans.ttf")),
                        font_size: bevy::prelude::FontSize::Px(80.0),
                        ..default()
                    },
                    TextColor(Color::WHITE),
                )]
            ),

            (//start button
                Button,
                AutoFocus,
                types::MenuButton,
                types::StartButton,
                Visibility::Visible,
                ZIndex(5),
                Node {
                    position_type: PositionType::Absolute,
                    height: Val::Percent(10.0),
                    width: Val::Percent(20.0),
                    left: Val::Percent(10.0),
                    top: Val::Percent(20.0),
                    ..default()
                },

                children![(
                    Text::new("Start"),
                    TextFont {
                        font: bevy::prelude::FontSource::Handle(asset_server.load("fonts/NotoSans.ttf")),
                        font_size: bevy::prelude::FontSize::Px(80.0),
                        ..default()
                    },
                    TextColor(Color::WHITE),
                )],
            ),

            (
                Button,
                AutoFocus,
                types::MenuButton,
                types::CreditsButton,
                Visibility::Visible,
                ZIndex(5),
                Node{
                    position_type: PositionType::Absolute,
                    height:Val::Percent(10.0),
                    width: Val::Percent(12.5),
                    left: Val::Percent(10.0),
                    top: Val::Percent(40.0),
                    ..default()
                },

                children![(
                    Text::new("Credits"),
                    TextFont { 
                        font: bevy::prelude::FontSource::Handle(asset_server.load("fonts/NotoSans.ttf")),
                        font_size: bevy::prelude::FontSize::Px(80.0),
                        ..default()
                    },

                    TextColor(Color::WHITE),
                )]
            ),
        ],
    ));
    //settings OVERLAY
    commands.spawn((
        types::SettingsPanel,
        ZIndex(99), //must overlay everything
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        Visibility::Hidden,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },

        children![
            (
                ZIndex(100), //overlays the actual overlay
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(60.0),
                    height: Val::Percent(60.0),
                    border: UiRect::all(Val::Px(8.0)),
                    border_radius: BorderRadius::all(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 1.0)),
                BorderColor::all(PURPLE),
            ),

            (
                //voice volume
                ZIndex(101),
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(20.0),
                    height: Val::Percent(10.0),
                    top: Val::Percent(20.0),
                    left: Val::Percent(10.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![
                    (
                        Text::new("Voice Volume: ".to_owned() + &(audio_settings.voice_volume*100.0).to_string() + "%"),
                        TextFont {
                            font: bevy::prelude::FontSource::Handle(asset_server.load("fonts/NotoSans.ttf")),
                            font_size: bevy::prelude::FontSize::Px(15.0),
                            ..default()
                        },
                    ),
                ]

            ),

            (
                ZIndex(101),
                types::AudioSettingsComponent::Voice,
                SliderValue(audio_settings.voice_volume),
                SliderPrecision(2),
                SliderRange::new(SLIDER_MIN, SLIDER_MAX),
                Slider {
                    track_click: Snap,
                    orientation: Horizontal,
                },
                Node {
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    height: Val::Percent(2.0),
                    width: Val::Percent(20.0),
                    overflow: Overflow {x: OverflowAxis::Visible, y: OverflowAxis::Visible},
                    ..default()
                },
                ImageNode {
                    image: asset_server.load("SettingsMenu/Slider.png"),
                    image_mode: NodeImageMode::Stretch,
                    ..default()
                },
                observe(slider_self_update),

                children![
                    (
                       Node {
                        position_type: PositionType::Absolute,
                        height: Val::Percent(100.0),
                        width: Val::Percent(100.0),
                        ..default()
                       },

                        children![
                            (
                                SliderThumb,
                                ZIndex(103),
                                Anchor::CENTER,
                                Node {
                                    position_type: PositionType::Absolute,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    width: Val::Percent(10.0),
                                    ..default()
                                },
                                ImageNode {
                                    image: asset_server.load("SettingsMenu/SliderThumb.png"),

                                    ..default()
                                },
                            )
                        ]
                    ),
                ]
            ),

            (
                //sfx volume
            ),

            (
                //music volume
            ),

            (
                //settings header
            ),

            (
                //save game
            ),

            (
                //load save (second button, but its just simpler)
            ),

            (
                //hard reset, (add confirmation)
            ),
        ],
    ));
    //ik i can make this more efficent but my lazy ass aint gonna do that rn
    commands.spawn((
        types::CreditsWindow,
        ZIndex(99),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
        Visibility::Hidden,
        Node{
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },

        children![
            (
                ZIndex(100),
                Node {
                    width: Val::Percent(25.0),
                    height: Val::Percent(20.0),
                    border: UiRect::all(Val::Px(12.0)),
                    border_radius: BorderRadius::all(Val::Px(12.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 1.0)),
                BorderColor::all(PURPLE),
            ),

            (
                ZIndex(101),
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(25.0),
                    height: Val::Percent(10.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                //second bounding box for text, so it doesn't spill out as much
                children![
                    (
                        Text::new("Art: Chibi|Neko and Rimi \nStory: Chibi|Neko and Rimi \nCoding: Anøm \nMusic: SgtSlippery"),
                        TextFont { 
                            font: bevy::prelude::FontSource::Handle(asset_server.load("fonts/NotoSans.ttf")),
                            font_size: bevy::prelude::FontSize::Px(30.0),
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ),
                ]
            )

        ],
    ));
    commands.insert_resource(types::TitleScreenState {
        state: types::TitleScreenSwap::BaseLibrary,
        timer: Timer::from_seconds(4.95, TimerMode::Once),
    });
}