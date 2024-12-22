use std::{array, f32::consts::PI};

use bevy::{prelude::*, render::mesh::TorusMeshBuilder};

use crate::config::hypocycloid_config;

pub struct HypocycloidTest;
impl Plugin for HypocycloidTest {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, config_gizmo);
        app.add_systems(Update, update_gizmo_config);
        app.add_systems(Update, Self::spawn_self);
    }
}

impl HypocycloidTest {

    /// Spawn the components of the hypocycloid
    fn spawn_self(
        mut _commands:Commands,
        mut _meshes: ResMut<Assets<Mesh>>,
        mut _materials: ResMut<Assets<StandardMaterial>>,
        mut draw: Gizmos
    ) {
        for i in 0..30 {
            draw.line_gradient(
                Vec3::splat(i as f32),
                Vec3::splat((i+1) as f32), 
                Color::srgb_from_array([i as f32 / 2.0, i as f32 / 4.0, i as f32 / 6.0]),
                Color::srgb_from_array([i as f32 / 4.0, i as f32 / 6.0, i as f32 / 8.0])
            );
        }

        // this doesn't work as i thought it would, only draws straight lines?
        draw.linestrip_gradient(
            // https://stackoverflow.com/questions/53688202/does-rust-have-an-equivalent-to-pythons-dictionary-comprehension-syntax
            (0..40).map(|i| (Vec3::new(i as f32 *2., i as f32 *3., i as f32*10.), Color::srgb(i as f32 /10., i as f32/10., i as f32/10.)))
            // (0..40).map(|i| (Vec3::splat(i as f32 * i as f32), Color::Srgba(Srgba::WHITE)))
        );



        // does nothing
        // (0..30).map(|i| draw.line(Vec3::splat(i as f32 * i as f32), Vec3::splat((i + 1) as f32 * (i+1) as f32), Color::WHITE));



        for j in 0..200{
            for i in 0..40{
                draw.line(Self::super_vec(i,j), Self::super_vec(i+1,j), Color::srgb_from_array([j as f32 /255., i as f32 /255., 1. - j as f32 /255.]))
            }        
        }

        // not a great choice because you need an array which has to of fixed size at compile time.
        // some hacks exist for getting a vec into an array
        draw.primitive_3d(&Polyline3d {
            vertices: array::from_fn::<Vec3, 30, _>(|i| Self::super_vec_f(i as f32*0.3, -20.0))
        }, Vec3::ZERO, Quat::default(), Color::srgb(1.0, 0.5, 0.0));
    }

    fn super_vec(i: u32, offset: u32) -> Vec3
    {
        let i = i as f32;
        Vec3::new(i*i + offset as f32, i*i*i, i*i*i*i)
    }

    fn super_vec_f(i: f32, offset: f32) -> Vec3
    {
        let i = i;
        Vec3::new(i*i + offset, i*i*i, i*i*i*i)
    }
}

//////////////////////////////////////////
//////////////////////////////////////////
//////////////////////////////////////////
#[derive(Resource)]
struct Points{
    points: Vec<Vec3>
}
#[derive(Component)]
struct OuterCircle;
#[derive(Component, Debug)]
struct RollingCircle;
#[derive(Component, Debug)]
struct TraceLine;
#[derive(Component, Debug)]
struct TracePoint;

pub struct Hypocycloid;
impl Plugin for Hypocycloid {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, update_gizmo_config);
        app.add_systems(Startup, Self::setup_scene);
        app.add_systems(Update, Self::update_pos);
        app.add_systems(Update, Self::draw_track);
    }
}

impl Hypocycloid {

    fn setup_scene(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {

        let outer_circle_radius = hypocycloid_config::OUTER_RAD;
        let torus_tube_radius = 0.1;
        let inner_circle_radius = hypocycloid_config::INNER_RAD;

        commands.insert_resource(Points { points: Vec::with_capacity(200_000)});

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(
                    TorusMeshBuilder {
                        torus: Torus{
                            major_radius: outer_circle_radius,
                            minor_radius: torus_tube_radius,
                        },
                        major_resolution: 100,
                        ..default()
                    }
                ),
                material: materials.add(Color::WHITE),
                transform : Transform::from_rotation(Quat::from_rotation_x(PI/2.0)),
                visibility: Visibility::Hidden,
                ..default()
            },
            OuterCircle,
        ));

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(
                    TorusMeshBuilder {
                        torus: Torus {
                            major_radius: inner_circle_radius,
                            minor_radius: torus_tube_radius
                        },
                        major_resolution: 100,
                        ..default()
                    }
                ),
                material: materials.add(Color::srgb(1.0, 0.0, 0.0)),
                // because of this rotation, you are rotating the coordinate frame of all the children.
                // so now instead of z pointing in /out of the screen, y does. 
                transform: Transform::from_xyz(outer_circle_radius -inner_circle_radius, 0.0, 0.0).with_rotation(Quat::from_rotation_x(PI/2.0)),
                ..default()
            },
            RollingCircle
        )).with_children(
            |parent| {
                parent.spawn((
                    PbrBundle {
                        mesh:  meshes.add(Cuboid::from_size(Vec3::new(inner_circle_radius, 0.1,0.1))),
                        material: materials.add(Color::WHITE),
                        transform : Transform::from_xyz(inner_circle_radius*0.5, 0.0,0.0), // 0,0,0 in this case is center of parent
                        ..default()
                    }, 
                    TraceLine
                )).with_children(
                    |parent| {
                        parent.spawn((
                            PbrBundle {
                                visibility: Visibility::Visible,
                                mesh : meshes.add(Sphere::default()),             
                                // from the reference frame of the stick. so need minus half length of stick or rad/2
                                transform: Transform::from_xyz(inner_circle_radius*0.5, 0.0, 0.0),
                                ..default()
                            },
                            TracePoint
                        ));
                    }
                );
            }
        );
    }
    
    fn update_pos(
        time: Res<Time>,
        mut query: Query<(&mut Children, &mut Transform), With<RollingCircle>>,
        mut child_query: Query<(&mut TraceLine, &mut Transform, &GlobalTransform), Without<RollingCircle>>,
        mut trace_point : Query<(&TracePoint, &GlobalTransform), (Without<RollingCircle>, Without<TraceLine>)>,
        mut draw : Gizmos,
        mut points: ResMut<Points>,
    ) {
        
        let mut children = query.single_mut();

        let rot_ang_circle = hypocycloid_config::CIRLCE_ROT_RATE + time.elapsed_seconds() / 100.;
        // let rot_ang_line = hypocycloid_config::LINE_ROT_RATE;
        let rot_ang_line = -hypocycloid_config::K* rot_ang_circle;

        children.1.rotate_around(Vec3::ZERO, Quat::from_rotation_z(rot_ang_circle));
        if ! children.1.rotation.is_normalized() {
            children.1.rotation = children.1.rotation.normalize();
            println!("children 1 not normalized");
        }
        draw.axes(*children.1, 2.0);

        // this is the traceline, bad names
        let mut traceline = child_query.single_mut().1;
        traceline.rotate_around(Vec3::ZERO, Quat::from_rotation_y(rot_ang_line));

        if ! traceline.rotation.is_normalized() {
            traceline.rotation = traceline.rotation.normalize();
            println!("traceline not normalized");
        }

        draw.axes(*child_query.single().2, 2.0);

        let mut tracepoint = trace_point.single_mut();
        // tracepoint.1.translation.x = 3.0*f32::sin(time.elapsed_seconds());
        // println!("{}", tracepoint.1.translation());

        points.points.push(tracepoint.1.translation());
        
        draw.axes(*tracepoint.1, 5.0 );
    }

    fn draw_track(
        points: Res<Points>,
        mut draw : Gizmos,
        time: Res<Time>
    ) {
        let points = &points.points;
        // let elasped = time.elapsed_seconds();
        let elasped = 0.0;
        let a60 = PI/3.0;
        let a120 = PI/3.0*2.0;
        let a180 = PI;
        let len_points = points.len();
        println!("{len_points}");
        // let len = points.len();
        for i in 1..len_points {
            let angle = elasped + (i as f32).to_radians();

            draw.line(points[i-1], points[i], Color::srgba(
                f32::sin(angle/2.0) *0.5+1., 
                // f32::sin(angle + a180)*0.5+1., 
                0.0, 
                f32::sin(angle/2.0 + a120)*0.5+1.,
                0.5
            ));
            // draw.line(points[i-1], points[i], Color::srgba(1.0,0.0,0.0, 0.5));
        }

    }

}

//////////////////////////////////////////
//////////////////////////////////////////
//////////////////////////////////////////


// SHARED GIZMO CONFIG CODE

fn update_gizmo_config(
    mut config_store: ResMut<GizmoConfigStore>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();

    if keyboard.pressed(KeyCode::Space) && !keyboard.pressed(KeyCode::ShiftLeft) {
        config.line_width += 30. * time.delta_seconds();
    }

    if keyboard.all_pressed([KeyCode::Space, KeyCode::ShiftLeft]){
        config.line_width -= 30. * time.delta_seconds();
        if config.line_width < 1.0 {
            config.line_width = 1.0;
        }
    }
}

fn config_gizmo(
    mut config_store: ResMut<GizmoConfigStore>,
) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();

    // might improve performance if this number was reduced or switched to a different mode
    config.line_joints = GizmoLineJoint::Miter;
    config.line_perspective = true; // for some reason this makes the line width affect it much much less
    config.line_width = 200.;
    // config.depth_bias = -0.2;
}

//////////////////////////////////////////
//////////////////////////////////////////
//////////////////////////////////////////



// https://wwwtyro.net/2019/11/18/instanced-lines.html
// https://www.reddit.com/r/bevy/comments/1ciwzb1/is_it_bad_to_use_gizmos_in_the_game/
// https://github.com/ForesightMiningSoftwareCorporation/bevy_polyline
// https://www.reddit.com/r/bevy/comments/1e04xk8/how_to_create_2d_object_from_arbitrary_list_of/
// seems like you either use polyline or gizmos. giszmos seems performant enough even tho they are redrawn every frame

// method one seems to be try gizmos? 
// if that doesnt work, try primitives https://docs.rs/bevy/0.14.2/bevy/math/primitives/index.html
// if that doesnt work try polyline
// if that doesnt work, try custom meshes with maybe a custom shader

