use bevy::prelude::*;


use smooth_bevy_cameras::controllers::unreal::UnrealCameraPlugin;
use smooth_bevy_cameras::LookTransformPlugin;

mod config;
use config::colors_config;
mod scene;

fn main() {
    App::new()
        // bevy's entire engine is written as plugins so maybe your stuff should be too? 
        .add_plugins((
            DefaultPlugins,
            LookTransformPlugin,
            UnrealCameraPlugin::default(),
        ))

        .insert_resource(ClearColor(colors_config::get_color(config::colors_config::BG_COLOR)))

        .add_systems(Startup, scene::setup)
        .add_systems(Startup, scene::draw_xyz)

        .run();


}