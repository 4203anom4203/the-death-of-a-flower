use bevy::{
    camera::visibility::RenderLayers, prelude::*
};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .run()
}

fn setup_camera(mut commands: Commands) {
    //higher render layer = on top :)

    commands.spawn((
        //first camera/bg camera
        Camera2d,
        Bg,
        Camera {
            //this is for background art, idk we need a resources folder
            order: 0,
            clear_color: ClearColorConfig::Custom(Color::WHITE),
            ..default()
        },

        RenderLayers::layer(0)
        //can only see things that are on the layer 0, otherwise it won't render.
    ));


}

//components
#[derive(Component)]
struct Bg;
//we will throw any backgrounds onto things with this component, aka the first camera
//