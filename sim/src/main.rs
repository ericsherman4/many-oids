use bevy::prelude::*;
use smooth_bevy_cameras::{controllers::unreal::UnrealCameraPlugin, LookTransformPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;



mod config;
mod scene;
mod animation;

fn main() {
    App::new()
        // bevy's entire engine is written as plugins so maybe your stuff should be too?
        .add_plugins((
            DefaultPlugins,
            // WorldInspectorPlugin::new(),
            LookTransformPlugin,
            UnrealCameraPlugin::default(),
            scene::CustomLightsPlugin,
            // scene::OriginPlugin,
            scene::CustomCameraPlugin,
            animation:: Hypocycloid,
        ))
        .run();
}
