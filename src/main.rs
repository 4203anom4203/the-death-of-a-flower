use bevy::{
    camera::visibility::RenderLayers, color::palettes::css::{BLACK, GREEN, RED, YELLOW}, input_focus::InputFocus, prelude::*, window::{Window, WindowMode}
};

const UI_BORDER_COLOR: Color = Color::srgba(0.749, 0.0, 1.0, 1.0);
fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<InputFocus>()
        .init_resource::<MenuState>()
        .add_systems(Startup, setup)
        .add_systems(Update, settings_button_system)
        .add_systems(Update, update_settings_menu)
        .run()
}


//setup func is for setting up title screen, everything else can move after
fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut window: Single<&mut Window>) {
    //higher render layer = on top :)

    window.mode = WindowMode::BorderlessFullscreen(
        MonitorSelection::Primary,
    );

    commands.spawn((
        //first camera/bg camera
        Camera2d,
        Camera {
            //This camera is for bg/sprites/every non interact item
            order: 0,
            clear_color: ClearColorConfig::Custom(Color::Srgba(BLACK)),
            //cursed as hell syntax
            ..default()
        },
        RenderLayers::layer(0)
        
    ));
    //we lowk may need more, background, character, text, buttons thats 4 so far, good enough
    commands.spawn((
        //Second camera for character sprites
        Camera2d,
        Camera {
            //again for characters and stuff they hold etc.
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },

        RenderLayers::layer(1)
    ));

    //image spawning
    commands.spawn((
        ZIndex(0), //bg
        Sprite {
            image: asset_server.load("TitleScreen/Library.png"),
            ..default()
        },
        
        //default renderlayer is 0
    ));
    //alr, this the settings icon.
    commands.spawn((
        //root node, contains the node
        //renders at top left, ill work on the animations for it later
        Button,
        //BorderColor::all(RED),
        SettingsButton,
        ZIndex(5), //buttons
        Node {
            position_type: PositionType::Absolute,
            width: Val::Px(50.0),
            height: Val::Px(50.0),
            left: Val::Px(5.0),
            top: Val::Px(5.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(5.0)), 
            ..default()
        }, 
        //if its not with the children macro it can't load a sprite because it will try
        //to assume the properties for a node/bounding box
        children![(
            //new imagenode with texture
            ImageNode::new(asset_server.load("settings.png")),

            Node {
                width: Val::Px(43.0),
                height: Val::Px(43.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            }
            
        )],
        
        //i almost forgot this
    ));

    commands.spawn((
        SettingsPanel,
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
            //new thingy
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
    menu: ResMut<MenuState>,
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
//my linter blew up because of unused code