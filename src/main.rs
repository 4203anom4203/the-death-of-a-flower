use bevy::{
    camera::visibility::RenderLayers, 
    prelude::*
};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_cameras)
        .run()
}

fn setup_cameras(mut commands: Commands) {
    //higher render layer = on top :)

    commands.spawn((
        //first camera/bg camera
        Camera2d,
        Camera {
            //this is for background art, idk we need a resources folder
            order: 0,
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        //Renderlayer is 0, its the default, make sure to match renderlayers
        
    ));
    //we lowk may need more, background, character, text, buttons thats 4 so far, good enough
    commands.spawn((
        //Second camera for character sprites
        Camera2d,
        Camera {
            //again for characters and stuff they hold etc.
            order: 1,
            clear_color: ClearColorConfig::Custom(Color::NONE),
            ..default()
        },

        RenderLayers::layer(1)
    ));


}

//components

