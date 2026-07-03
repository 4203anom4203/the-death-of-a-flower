use bevy::{
    color::palettes::css::BLACK, input_focus::AutoFocus, prelude::*, window::{Window, WindowMode}, 
};
use crate::derive;

//setup func is for setting up title screen, everything else can move after

pub const PURPLE: Color = Color::srgba(0.749, 0.0, 1.0, 1.0);
static VOICE_VOLUME: u32 = 100;
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut window: Single<&mut Window>,
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
        derive::WorldCamera,
        Camera {
            //This camera is for everything now
            order: 0,
            clear_color: ClearColorConfig::Custom(Color::Srgba(BLACK)),
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
                derive::TitleBackgroundImage,
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
                derive::SettingsButton,
                derive::MenuButton,
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
                derive::MenuButton,
                derive::StartButton,
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
                derive::MenuButton,
                derive::CreditsButton,
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
        derive::SettingsPanel,
        ZIndex(99), //must overlay everything
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        Visibility::Hidden,
        Node {
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
                    width: Val::Percent(30.0),
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
                    width: Val::Percent(75.0),
                    height: Val::Percent(10.0),
                    top: Val::Percent(25.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![
                    (
                        Text::new(VOICE_VOLUME.to_string()),
                        TextFont {
                            font: bevy::prelude::FontSource::Handle(asset_server.load("fonts/NotoSans.ttf")),
                            font_size: bevy::prelude::FontSize::Px(60.0),
                            ..default()
                        },
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
        derive::CreditsWindow,
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
    commands.insert_resource(derive::TitleScreenState {
        state: derive::TitleScreenSwap::BaseLibrary,
        timer: Timer::from_seconds(4.95, TimerMode::Once),
    });
}