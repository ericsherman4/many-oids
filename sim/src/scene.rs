use bevy::prelude::*;
use smooth_bevy_cameras::controllers::unreal::{UnrealCameraBundle, UnrealCameraController};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::config::{cam_config, colors_config, lights_config, origin_config};

//////////////////////////////////////////////////
/// LIGHTING
//////////////////////////////////////////////////
pub struct CustomLightsPlugin;
impl Plugin for CustomLightsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(colors_config::get_color(
            lights_config::BG_COLOR,
        )));
        app.add_systems(Startup, Self::create_light);
    }
}

impl CustomLightsPlugin {
    fn create_light(mut commands: Commands, mut gizmo_store: ResMut<GizmoConfigStore>) {
        // Light
        let point_light_bundle_1 = SpotLightBundle {
            spot_light: SpotLight {
                shadows_enabled: true,
                shadow_depth_bias: 0.3,
                intensity: 20_000_000.,
                range: 50.,
                color: Color::Srgba(Srgba::WHITE),
                ..default()
            },
            transform: Transform::from_translation(lights_config::POS_1)
                .looking_at(lights_config::LOOKING_AT_1, Vec3::Y),
            ..default()
        };

        // Second light
        let point_light_bundle_2 = SpotLightBundle {
            spot_light: SpotLight {
                shadows_enabled: true,
                shadow_depth_bias: 0.3,
                intensity: 20_000_000.,
                range: 50.,
                color: Color::Srgba(Srgba::WHITE),
                ..default()
            },
            transform: Transform::from_translation(lights_config::POS_2)
                .looking_at(lights_config::LOOKING_AT_2, Vec3::Y),
            ..default()
        };

        // Light spawn
        commands.spawn(point_light_bundle_1);
        commands.spawn(point_light_bundle_2);

        // Gimzo config
        if lights_config::GIZMOS_ON {
            let (_, light_config) = gizmo_store.config_mut::<LightGizmoConfigGroup>();
            light_config.draw_all = true;
            light_config.color = LightGizmoColor::Varied;
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
pub struct CustomCameraPlugin;
impl Plugin for CustomCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::create_camera);
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
            UnrealCameraController::default(),
            STARTING_CAM_POS,
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        );

        commands.spawn(bevy_camera).insert(unreal_camera);
    }
}
