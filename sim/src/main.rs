use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use smooth_bevy_cameras::{controllers::unreal::UnrealCameraPlugin, LookTransformPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;



mod config;
mod scene;
mod animation;

fn main() {
    App::new()
        // bevy's entire engine is written as plugins so maybe your stuff should be too?
        .add_plugins((
            DefaultPlugins.set(
                // https://github.com/bevyengine/bevy/blob/v0.14.2/examples/window/window_settings.rs
                WindowPlugin {
                    primary_window: Some((Window {
                        mode: bevy::window::WindowMode::BorderlessFullscreen,
                        name: Some("Bevy Sim".into()),
                        ..default()
                    })),
                    ..default()
                }
            ),
            // WorldInspectorPlugin::new(),
            LookTransformPlugin,
            UnrealCameraPlugin::default(),
            scene::CustomLightsPlugin,
            // scene::OriginPlugin,
            scene::CustomCameraPlugin,
            animation:: Hypocycloid,
        ))
        .add_systems(Update, exit_app.run_if(input_just_pressed(KeyCode::Escape)))
        .run();
}

fn exit_app(
    mut exit: EventWriter<AppExit>
) {
    println!("App exist triggered with escape...exiting..");
    exit.send(AppExit::Success);
}