use bevy::{
    camera::visibility::RenderLayers,
    color::palettes::css::BLACK,
    input_focus::InputFocus,
    prelude::*
};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(InputFocus::default())
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .run()
}

//setup func is for setting up title screen, everything else can move after
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //higher render layer = on top :)

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

    //image spawning
    commands.spawn((
        Sprite {
            image: asset_server.load("image-not-found.png"),
            ..default()
        },
        RenderLayers::layer(0)
        //default renderlayer is 0
    ));

}

fn button_system (
    mut input_focus: ResMut<InputFocus>,
    mut interaction_query: Query<(Entity, &Interaction), Changed<Interaction>>,
) {
    for (entity, interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity);

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
//components
