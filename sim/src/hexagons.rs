use std::{f32::consts::PI};

use bevy::{ 
    prelude::*,
    pbr::{CascadeShadowConfigBuilder, NotShadowCaster},
    core_pipeline::{
        bloom::{BloomCompositeMode, BloomSettings},
        tonemapping::Tonemapping,
        experimental::taa::{TemporalAntiAliasPlugin},
    },
    color::palettes,
};

pub struct Hexagons;
impl Plugin for Hexagons {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::spawn_hexagons);
        // app.add_systems(Update,Self::update_bloom_settings);
        app.add_plugins(TemporalAntiAliasPlugin);
    }
}

impl Hexagons {
    fn spawn_hexagons(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        camera: Query<(Entity, Option<&mut BloomSettings>), With<Camera>>

    ) {

        const LAYERS : u32 = 40;
        const SIDE_LEN : f32 = 2.;
        const SPACING: f32 = 0.15;
        let hexagon = RegularPolygon {
            circumcircle: Circle::new(SIDE_LEN),
            sides: 6,
        };
        let hexagon_thickness = 10.0;
        let horz_distance:f32  = hexagon.inradius()*2.0 + SPACING;
        let half_horz_distance:f32 = horz_distance * 0.5;
        let height_distance:f32 = horz_distance *  f32::sqrt(3.)/ f32::sqrt(4.);
        let extrusion = Extrusion::new(hexagon, hexagon_thickness); // change height later

        let material = StandardMaterial {
            reflectance: 0.13,
            perceptual_roughness: 0.0,
            base_color: Color::BLACK.into(),
            emissive:LinearRgba::BLACK,
            diffuse_transmission: 1.0,
            specular_transmission: 0.9,
            thickness: 3.,
            ior:1.77,
            ..default()
        };

        let mut transform =  Transform::from_translation(Vec3::ZERO);
        transform.rotate_x(PI*0.5);
        transform.rotate_y(PI/6.);
        
        const TOTAL_LEVELS: u32= LAYERS*2 -1;
        if TOTAL_LEVELS == 1 {
            commands.spawn(
                PbrBundle{
                    mesh: meshes.add(extrusion.clone()),
                    material: materials.add(material.clone()),
                    transform: transform,
                    ..default()
                }
            );
            return
        }


        let grid_x_offset = SIDE_LEN;
        let grid_z_offset = hexagon.inradius();
        // else generate the grid
        for x in 0..TOTAL_LEVELS {
            let num_in_row = if x%2 == 0 {TOTAL_LEVELS-1} else {TOTAL_LEVELS};
            let row_offset = if x%2 == 0 {half_horz_distance} else {0.0};
            for z in 0..(num_in_row){
                commands.spawn(
                    PbrBundle{
                        mesh: meshes.add(extrusion.clone()),
                        material: materials.add(material.clone()),
                        transform: transform.with_translation(Vec3::new(
                                grid_x_offset+ x as f32*height_distance, 
                                -hexagon_thickness*0.5+1.5 -(x as f32*0.5).sin()*0.5 + (z as f32*0.5).cos()*0.5 + ((x*z) as f32*0.4).cos()*0.5, 
                                // -hexagon_thickness*0.5+0.5,
                                grid_z_offset + row_offset +  z as f32*horz_distance
                            )),
                        ..default()
                    }
                );
            }
        }


    
    let emissive_strength = 10.0;    
    // let center_x = grid_x_offset+ LAYERS as f32*height_distance;
    // let center_z = grid_z_offset +  LAYERS as f32*horz_distance;
    let center_x = (height_distance*TOTAL_LEVELS as f32) /2.0 ;
    let center_z = (horz_distance*TOTAL_LEVELS as f32) / 2.0;
    let mesh_x_width = height_distance*TOTAL_LEVELS as f32 -10.;
    let mesh_z_width = horz_distance*TOTAL_LEVELS as f32- (SIDE_LEN - hexagon.inradius())*2.0 -10.;
    // ground plane
    commands.spawn(
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(mesh_x_width,mesh_z_width ).subdivisions(10)),
            material: materials.add(
                StandardMaterial {
                    emissive: LinearRgba::rgb(emissive_strength, 0.0, 0.0),
                    ..default()
                }
            ),
            transform: Transform::from_xyz(center_x,0.0,center_z),
            ..default()

        }
    );

    // Configure a properly scaled cascade shadow map for this scene (defaults are too large, mesh units are in km)
    let cascade_shadow_config = CascadeShadowConfigBuilder {
        first_cascade_far_bound: 0.3,
        maximum_distance: 3.0,
        ..default()
    }
    .build();

    // Sun
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::srgb(0.98, 0.95, 0.82),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0)
            .looking_at(Vec3::new(-0.15, -0.05, 0.25), Vec3::Y),
        cascade_shadow_config,
        ..default()
    });


    // example instructions
    commands.spawn(
        TextBundle::from_section("", TextStyle::default()).with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }),
    );

    }

    fn update_bloom_settings(
        mut camera: Query<(Entity, Option<&mut BloomSettings>), With<Camera>>,
        mut text: Query<&mut Text>,
        mut commands: Commands,
        keycode: Res<ButtonInput<KeyCode>>,
        time: Res<Time>,
    ) {
        let bloom_settings = camera.single_mut();
        let mut text = text.single_mut();
        let text = &mut text.sections[0].value;
    
        match bloom_settings {
            (entity, Some(mut bloom_settings)) => {
                *text = "BloomSettings (Toggle: Space)\n".to_string();
                text.push_str(&format!("(Q/A) Intensity: {}\n", bloom_settings.intensity));
                text.push_str(&format!(
                    "(W/S) Low-frequency boost: {}\n",
                    bloom_settings.low_frequency_boost
                ));
                text.push_str(&format!(
                    "(E/D) Low-frequency boost curvature: {}\n",
                    bloom_settings.low_frequency_boost_curvature
                ));
                text.push_str(&format!(
                    "(R/F) High-pass frequency: {}\n",
                    bloom_settings.high_pass_frequency
                ));
                text.push_str(&format!(
                    "(T/G) Mode: {}\n",
                    match bloom_settings.composite_mode {
                        BloomCompositeMode::EnergyConserving => "Energy-conserving",
                        BloomCompositeMode::Additive => "Additive",
                    }
                ));
                text.push_str(&format!(
                    "(Y/H) Threshold: {}\n",
                    bloom_settings.prefilter_settings.threshold
                ));
                text.push_str(&format!(
                    "(U/J) Threshold softness: {}\n",
                    bloom_settings.prefilter_settings.threshold_softness
                ));
    
                if keycode.just_pressed(KeyCode::Space) {
                    commands.entity(entity).remove::<BloomSettings>();
                }
    
                let dt = time.delta_seconds();
    
                if keycode.pressed(KeyCode::KeyA) {
                    bloom_settings.intensity -= dt / 10.0;
                }
                if keycode.pressed(KeyCode::KeyQ) {
                    bloom_settings.intensity += dt / 10.0;
                }
                bloom_settings.intensity = bloom_settings.intensity.clamp(0.0, 1.0);
    
                if keycode.pressed(KeyCode::KeyS) {
                    bloom_settings.low_frequency_boost -= dt / 10.0;
                }
                if keycode.pressed(KeyCode::KeyW) {
                    bloom_settings.low_frequency_boost += dt / 10.0;
                }
                bloom_settings.low_frequency_boost = bloom_settings.low_frequency_boost.clamp(0.0, 1.0);
    
                if keycode.pressed(KeyCode::KeyD) {
                    bloom_settings.low_frequency_boost_curvature -= dt / 10.0;
                }
                if keycode.pressed(KeyCode::KeyE) {
                    bloom_settings.low_frequency_boost_curvature += dt / 10.0;
                }
                bloom_settings.low_frequency_boost_curvature =
                    bloom_settings.low_frequency_boost_curvature.clamp(0.0, 1.0);
    
                if keycode.pressed(KeyCode::KeyF) {
                    bloom_settings.high_pass_frequency -= dt / 10.0;
                }
                if keycode.pressed(KeyCode::KeyR) {
                    bloom_settings.high_pass_frequency += dt / 10.0;
                }
                bloom_settings.high_pass_frequency = bloom_settings.high_pass_frequency.clamp(0.0, 1.0);
    
                if keycode.pressed(KeyCode::KeyG) {
                    bloom_settings.composite_mode = BloomCompositeMode::Additive;
                }
                if keycode.pressed(KeyCode::KeyT) {
                    bloom_settings.composite_mode = BloomCompositeMode::EnergyConserving;
                }
    
                if keycode.pressed(KeyCode::KeyH) {
                    bloom_settings.prefilter_settings.threshold -= dt;
                }
                if keycode.pressed(KeyCode::KeyY) {
                    bloom_settings.prefilter_settings.threshold += dt;
                }
                bloom_settings.prefilter_settings.threshold =
                    bloom_settings.prefilter_settings.threshold.max(0.0);
    
                if keycode.pressed(KeyCode::KeyJ) {
                    bloom_settings.prefilter_settings.threshold_softness -= dt / 10.0;
                }
                if keycode.pressed(KeyCode::KeyU) {
                    bloom_settings.prefilter_settings.threshold_softness += dt / 10.0;
                }
                bloom_settings.prefilter_settings.threshold_softness = bloom_settings
                    .prefilter_settings
                    .threshold_softness
                    .clamp(0.0, 1.0);
            }
    
            (entity, None) => {
                *text = "Bloom: Off (Toggle: Space)".to_string();
    
                if keycode.just_pressed(KeyCode::Space) {
                    commands.entity(entity).insert(BloomSettings::NATURAL);
                }
            }
        }
    }

}

