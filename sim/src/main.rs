use bevy::prelude::*;


use smooth_bevy_cameras::controllers::unreal::UnrealCameraPlugin;
use smooth_bevy_cameras::LookTransformPlugin;

mod config;
use config::scene;

fn main() {
    App::new()
        // bevy's entire engine is written as plugins so maybe your stuff should be too? 
        .add_plugins((
            DefaultPlugins,
            LookTransformPlugin,
            UnrealCameraPlugin::default(),
        ))

        .insert_resource(ClearColor(scene::get_color(scene::BG_COLOR)))

        .run();


}