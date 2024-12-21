use bevy::prelude::*;
use smooth_bevy_cameras::{controllers::unreal::UnrealCameraPlugin, LookTransformPlugin};

mod config;
mod scene;
mod animation;

fn main() {
    App::new()
        // bevy's entire engine is written as plugins so maybe your stuff should be too?
        .add_plugins((
            DefaultPlugins,
            LookTransformPlugin,
            UnrealCameraPlugin::default(),
            scene::CustomLightsPlugin,
            scene::OriginPlugin,
            scene::CustomCameraPlugin,
            // animation::HypocycloidTest,
            animation:: Hypocycloid,
        ))
        .run();
}
