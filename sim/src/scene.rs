use std::{f32::consts::PI, time::Duration};

use bevy::{input::common_conditions::input_just_pressed, prelude::*, time::common_conditions::{once_after_delay, repeating_after_delay}};
use smooth_bevy_cameras::controllers::unreal::{UnrealCameraBundle, UnrealCameraController};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::config::{cam_config, colors_config, hypocycloid_config, lights_config, origin_config};

use bevy::ecs::system::SystemId;

//////////////////////////////////////////////////
/// LIGHTING
//////////////////////////////////////////////////

#[derive(Resource)]
struct LightRadiusAnim{
    start_time: f32,
    state: i32,
    running: bool,
}

pub struct CustomLightsPlugin;
impl Plugin for CustomLightsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(colors_config::get_color(
            lights_config::BG_COLOR,
        )));
        app.insert_resource(
            LightRadiusAnim {
                start_time: 0.0,
                running: false,
                state: 0,
        });
        app.add_systems(Startup, Self::create_light);
        app.add_systems(Update, Self::animate_light_radius);
    }
}

impl CustomLightsPlugin {
    fn create_light(
        mut commands: Commands, 
        mut gizmo_store: ResMut<GizmoConfigStore>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        // Light
        let point_light_bundle_1 = SpotLightBundle {
            spot_light: SpotLight {
                shadows_enabled: true,
                shadow_depth_bias: 0.3,
                intensity: 20_000_000.,
                range: 500.0,
                radius: 20.0,
                color: colors_config::get_color("C84C05"),
                ..default()
            },
            transform: Transform::from_translation(lights_config::POS_1)
                .looking_at(lights_config::LOOKING_AT_1, Vec3::Y),
            ..default()
        };

        // Light spawn
        commands.spawn(point_light_bundle_1);

        // Gimzo config
        if lights_config::GIZMOS_ON {
            let (_, light_config) = gizmo_store.config_mut::<LightGizmoConfigGroup>();
            light_config.draw_all = true;
            light_config.color = LightGizmoColor::Varied;
        }

        // this does not belong here but I am being lazy
        // commands.spawn(
        //     PbrBundle {
        //         mesh: meshes.add(Plane3d { half_size: Vec2::new(200.0, 200.0), normal: Dir3::NEG_Z}),
        //         material: materials.add(colors_config::get_color("1f1f1f")),
        //         ..default()
        //     }
        // );
    }

    fn animate_light_radius(
        mut lights: Query<&mut SpotLight>,
        time: Res<Time>,
        mut anim: ResMut<LightRadiusAnim>,
        keyboard: Res<ButtonInput<KeyCode>>,

    ) {
        if anim.running == false && keyboard.just_pressed(KeyCode::KeyI){
            anim.running = true;
            anim.start_time = time.elapsed_seconds();
            anim.state = 0;
            println!("starting light animation");
        } 
        // animation is already running
        else if anim.running {
            let x  = time.elapsed_seconds()-anim.start_time;
            let target_start_y_val = 20.0;
            let amp = 35.0;
            // calculate phase so that 
            // at t=0, it will start at about a y value of `target_start_y_val`
            let phase = f32::asin(-1.0*target_start_y_val / amp );
            for mut light in lights.iter_mut() {

                light.radius = -1.0*amp* f32::sin( x / 3.0 + phase);
                if light.radius < -29.0 && anim.state == 0 {
                    anim.state+=1;
                    println!("state is now 1");
                }else if light.radius > -28.0 && anim.state == 1 {
                    anim.running = false;
                    println!("ending light animation");
                }
            }
        }


    }
}

//////////////////////////////////////////////////
/// ORIGIN
//////////////////////////////////////////////////

#[derive(EnumIter)]
enum Axis {
    X,
    Y,
    Z,
}

pub struct OriginPlugin;
impl Plugin for OriginPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::draw_origin);
    }
}

impl OriginPlugin {
    /// Draw the origin and xyz axes
    pub fn draw_origin(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        // Origin
        commands.spawn(PbrBundle {
            mesh: meshes.add(
                Sphere {
                    radius: origin_config::ORIGIN_SPHERE_RADIUS,
                }
                .mesh()
                .uv(32, 18),
            ),
            material: materials.add(Color::WHITE),
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        });

        for variant in Axis::iter() {
            commands.spawn(Self::create_axis(variant, &mut meshes, &mut materials));
        }
    }

    /// Helper function to generate the xyz axes in each direction
    fn create_axis(
        direction: Axis,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> PbrBundle {
        let cuboid_dim: Vec3;
        let adjusted_position: Vec3;
        let color: Color;

        const LENGTH: f32 = origin_config::AXIS_LENGTH;
        const GIRTH: f32 = origin_config::AXIS_GIRTH;
        const HALF_LENGTH: f32 = LENGTH * 0.5;

        match direction {
            Axis::X => {
                cuboid_dim = Vec3::new(LENGTH, GIRTH, GIRTH);
                adjusted_position = Vec3::new(HALF_LENGTH, 0., 0.);
                color = origin_config::COLOR_X;
            }
            Axis::Y => {
                cuboid_dim = Vec3::new(GIRTH, LENGTH, GIRTH);
                adjusted_position = Vec3::new(0., HALF_LENGTH, 0.);
                color = origin_config::COLOR_Y;
            }
            Axis::Z => {
                cuboid_dim = Vec3::new(GIRTH, GIRTH, LENGTH);
                adjusted_position = Vec3::new(0., 0., HALF_LENGTH);
                color = origin_config::COLOR_Z;
            }
        }

        PbrBundle {
            mesh: meshes.add(Cuboid::from_size(cuboid_dim)),
            material: materials.add(color),
            transform: Transform::from_translation(adjusted_position),
            ..default()
        }
    }
}

//////////////////////////////////////////////////
/// CAMERAS
//////////////////////////////////////////////////

#[derive(Resource)]
struct CameraAnimState {
    state: i8,
}

pub struct CustomCameraPlugin;
impl Plugin for CustomCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraAnimState {state: 0});
        app.add_systems(Startup, Self::create_camera);
        app.add_systems(Update, Self::lock_camera.run_if(input_just_pressed(KeyCode::KeyL)));
        // app.add_systems(Update, Self::animate_camera);
        

    }
}



impl CustomCameraPlugin {
    /// Create the unreal engine camera object in the scene
    fn create_camera(mut commands: Commands) {
        const STARTING_CAM_POS: Vec3 = cam_config::POS;
        const TARGET: Vec3 = cam_config::LOOKING_AT;

        println!(
            "Camera is starting at {} and pointing at {}",
            TARGET, STARTING_CAM_POS
        );

        let bevy_camera = Camera3dBundle {
            projection: PerspectiveProjection { ..default() }.into(),
            // looking at is how to orient
            // y is up in bevy
            transform: Transform::from_translation(STARTING_CAM_POS).looking_at(TARGET, Vec3::Y),
            ..default()
        };

        let unreal_camera = UnrealCameraBundle::new(
            UnrealCameraController {
                // settings for 60 fps monitor
                // smoothing_weight : 0.99,
                // rotate_sensitivity: Vec2::splat(0.015),
                ..default()
            },
            STARTING_CAM_POS,
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        );

        commands.spawn(bevy_camera).insert(unreal_camera);
    }

    fn lock_camera(
        mut camera_controller: Query<&mut UnrealCameraController>,
    ) {
        for mut cam in camera_controller.iter_mut() {
            cam.enabled ^= true;
            println!("Cam is now {}", cam.enabled)
        }

    }

    fn animate_camera(
        mut query: Query<(&mut Transform, &Camera)>,
        mut state: ResMut<CameraAnimState>
    ) {
        let (mut camera,_) = query.single_mut();
        // you have to unlock the unrealcamera controller to do this because it overrides the position of the normal camera. which
        // is kind of gret because that means i can trigger this when i want.
        let interval :f32 = 0.2;
        match state.state {
            0 => {
                camera.translation.z += interval;
                if camera.translation.z > -8.0 {state.state = 1}
            }
            1 => {
                camera.translation.z -= interval;
                if camera.translation.z < cam_config::POS.z {state.state =2}

            } 
            _ => ()
        }
       
    }



}
