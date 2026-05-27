use bevy::{
    camera::visibility::RenderLayers, prelude::*
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
        Bg,
        Camera {
            //this is for background art, idk we need a resources folder
            order: 0,
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },

        RenderLayers::layer(0)
        //can only see things that are on the layer 0, otherwise it won't render.
    ));

    commands.spawn((
        //Second camera for character sprites
        Camera2d,
        Char,
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
#[derive(Component)]
struct Bg;
//we will throw any backgrounds onto things with this component, aka the first camera
#[derive(Component)]
struct Char;
