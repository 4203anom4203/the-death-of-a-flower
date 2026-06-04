use bevy::{
    color::palettes::css::BLACK, input_focus::InputFocus, prelude::*, window::{Window, WindowMode}
};
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
const UI_BORDER_COLOR: Color = Color::srgba(0.749, 0.0, 1.0, 1.0);

fn main() -> AppExit {
    App::new()
        .add_plugins((
            EmbeddedAssetPlugin {
                mode: PluginMode::ReplaceDefault,
            },
            DefaultPlugins,
        ))
        .init_resource::<InputFocus>()
        .init_resource::<MenuState>()
        .add_systems(Startup, setup)
        .add_systems(Update, settings_button_system)
        .add_systems(Update, update_settings_menu)
        .add_systems(Update, update_title_background)
        .add_systems(Update, settings_menu_keybind)
        .run()
}

//setup func is for setting up title screen, everything else can move after
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut window: Single<&mut Window>,
) {
    //higher render layer = on top :)
    //TODO: add buttons, in ddlc style
    //TODO: settings buttons in menup
    //TODO: SAVE FILES
    window.mode = WindowMode::BorderlessFullscreen(
        MonitorSelection::Primary,
    );

    commands.spawn((
        //camera
        Camera2d,
        WorldCamera,
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
                TitleBackgroundImage,
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
                SettingsButton,
                ZIndex(5), //simple ui button, but the sprites will render on 3 or something
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    width: Val::Px(50.0),
                    height: Val::Px(50.0),
                    ..default()
                },

                children![(
                    ImageNode {
                        image: asset_server.load("settings.png"),
                        image_mode: NodeImageMode::Auto,
                        ..default() //settings
                    },
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                )]
            ),

            (
                Button,
                TitleButton,
                ZIndex(5), //doesn't get close to the settings button so we should be fine
                Node {
                    width: Val::Percent(20.0),
                    height: Val::Percent(5.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(1.0, 1.0, 1.0)),
                children![(
                    Text::new("Start"),
                    TextFont {
                        font: asset_server.load("fonts/comic_sans.ttf"),
                        font_size: 35.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.749, 0.0, 1.0)),
                )]
            ),
        ],
    ));

    commands.spawn((
        SettingsPanel,
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

        children![(
            ZIndex(100), //overlays the actual overlay
            Node {
                width: Val::Percent(30.0),
                height: Val::Percent(60.0),
                border: UiRect::all(Val::Px(12.0)),
                border_radius: BorderRadius::all(Val::Px(12.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 1.0)),
            BorderColor::all(UI_BORDER_COLOR),
        )],
    ));

    commands.insert_resource(TitleScreenState {
        state: TitleScreenSwap::BaseLibrary,
        timer: Timer::from_seconds(4.95, TimerMode::Once),
    });
}

fn settings_button_system (
    mut input_focus: ResMut<InputFocus>,
    mut state: ResMut<MenuState>,
    mut interaction_query: Query<(Entity, &Interaction, /* &mut BorderColor,*/ &Button, &SettingsButton), Changed<Interaction>>,
) {
    for (entity, interaction, /*mut border_color,*/ &Button, &SettingsButton) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity);
                //*border_color = BorderColor::all(GREEN);
                state.current_menu = match state.current_menu {
                Menu::None => Menu::Settings,
                Menu::Settings => Menu::None, //settings overlay tobble
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

fn update_settings_menu (
    menu: Res<MenuState>,
    mut panel_query: Query<&mut Visibility, With<SettingsPanel>>,
) {
    for mut visibility in &mut panel_query {
        *visibility = if menu.current_menu == Menu::Settings {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    
}

fn settings_menu_keybind (
    input: Res<ButtonInput<KeyCode>>,
    mut menu: ResMut<MenuState>,
) {
    if input.just_pressed(KeyCode::Escape)&&menu.current_menu == Menu::Settings {
        menu.current_menu = Menu::None;
    } else if input.just_pressed(KeyCode::Escape)&&menu.current_menu == Menu::None {
        menu.current_menu = Menu::Settings;
    }
}

fn update_title_background (
    mut titlescreen: ResMut<TitleScreenState>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut query: Query<&mut ImageNode, With<TitleBackgroundImage>>
) {
    titlescreen.timer.tick(time.delta());

    if titlescreen.timer.just_finished() {
        for mut image_node in &mut query {
            match titlescreen.state {

                TitleScreenSwap::Transition1 => {
                    image_node.image = asset_server.load("TitleScreen/Glitch_Frame.png");
                    titlescreen.timer = Timer::from_seconds(0.05, TimerMode::Once);
                    titlescreen.state = TitleScreenSwap::BaseLibrary;
                }
                TitleScreenSwap::BaseLibrary => {
                    image_node.image = asset_server.load("TitleScreen/Library_Soft.png");
                    titlescreen.timer = Timer::from_seconds(4.95, TimerMode::Once);
                    titlescreen.state = TitleScreenSwap::Transition2;
                    
                }

                TitleScreenSwap::Transition2 => {
                    image_node.image = asset_server.load("TitleScreen/Glitch_Frame.png");
                    titlescreen.timer = Timer::from_seconds(0.05, TimerMode::Once);
                    titlescreen.state = TitleScreenSwap::DecayLibrary;
                }

                TitleScreenSwap::DecayLibrary => {
                    image_node.image = asset_server.load("TitleScreen/Decay_Library_Tint.png");
                    titlescreen.timer = Timer::from_seconds(4.95, TimerMode::Once);
                    titlescreen.state = TitleScreenSwap::Transition1;
                }

                
            }

        }
    }
}

#[derive(Default, PartialEq)]
enum Menu {
    #[default]
    None,
    Settings,
    //can add shit
}
//resources
#[derive(Resource, Default)]
struct MenuState {
    current_menu: Menu,
}

//components
#[derive(Component)]
struct SettingsButton;

#[derive(Component)]
struct SettingsPanel;

#[derive(Resource, Default)]
enum TitleScreenSwap {
    #[default]
    BaseLibrary,
    DecayLibrary,
    Transition1,
    Transition2,
}

#[derive(Resource, Default)]
struct TitleScreenState {
    state: TitleScreenSwap,
    timer: Timer,
}

#[derive(Component)]
struct WorldCamera;

#[derive(Component)]
struct TitleBackgroundImage;

#[derive(Component)]
struct TitleButton;
//my linter blew up because of unused code