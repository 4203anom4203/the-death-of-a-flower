use bevy::{
    camera::visibility::RenderLayers,
    color::palettes::css::{BLACK, RED, YELLOW, GREEN}, //Don't mind these its for debug
    input_focus::InputFocus,
    window::Window,
    window::WindowMode,
    prelude::*
};


fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<InputFocus>()
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
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
            //this is for background art, idk we need a resources folder
            order: 0,
            clear_color: ClearColorConfig::Custom(Color::Srgba(BLACK)),
            //cursed as hell syntax
            ..default()
        },
        RenderLayers::layer(0)
        //Renderlayer is 0, its the default, make sure to match renderlayers
        
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

    commands.spawn((
        //third camera for text box
        Camera2d,
        Camera {
            //textboxes etc
            order: 2,
            clear_color: ClearColorConfig::None,
            ..default()
        },

        RenderLayers::layer(2)
    ));
    
    commands.spawn((
        //last camera hopefully for ui/overlay and buttons
        Camera2d,
        Camera {
            order: 3,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        RenderLayers::layer(3)
    ));
    //we need another one for the settings actual window/popup

    commands.spawn((
        Camera2d, 
        Camera {
            order: 4,
            clear_color: ClearColorConfig::None,
            ..default()
        },

        RenderLayers::layer(4)
    ));    
    //image spawning
    commands.spawn((
        Sprite {
            image: asset_server.load("placeholder.png"),
            ..default()
        },
        RenderLayers::layer(0)
        //default renderlayer is 0
    ));
    //alr, this the settings icon.
    commands.spawn((
        //root node, contains the node
        //renders at top left, ill work on the animations for it later
        Button,
        //BorderColor::all(RED),
        SettingsButton,
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
        RenderLayers::layer(3)
        
        //i almost forgot this
    ));

}

fn button_system (
    mut input_focus: ResMut<InputFocus>,
    mut interaction_query: Query<(Entity, &Interaction, /* &mut BorderColor,*/ &Button, &SettingsButton), Changed<Interaction>>,
) {
    for (entity, interaction, /*mut border_color,*/ &Button, &SettingsButton) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity);
                //*border_color = BorderColor::all(GREEN);
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
//components
#[derive(Component)]
struct SettingsButton;

/*
use 
#[derive(Component)]
enum MenuButton {
    Settings,
    Save,
    Load,
    Quit,
}

    when we add more than one button
*/