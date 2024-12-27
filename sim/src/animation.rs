use std::{f32::consts::PI, time::Duration};
use bevy::{prelude::*, render::mesh::TorusMeshBuilder, time::common_conditions::repeating_after_delay};
use crate::config::hypocycloid_config;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;



#[derive(Resource)]
struct Points{
    //TODO: maybe this should be part of tracepoint so you remove the need for everything to be a singleton
    points: Vec<Vec3> 
}
/// A resource that will override the color of the hypocycloid 
#[derive(Reflect,Resource, Default)]
#[reflect(Resource)]
struct OverrideColor {
    /// Will override the color if true
    overrode: bool,

    /// The override color
    override_color: Color,
}

#[derive(Component)]
struct OuterCircle;
#[derive(Component, Debug)]
struct RollingCircle;
#[derive(Component, Debug)]
struct TraceLine;
#[derive(Component, Debug)]
struct TracePoint;

#[derive(Component, Debug)]
struct HypocycloidTracking {
    /// the center of the outer circle
    outer_circle_xform: Transform,

    /// the center of the inner circle
    inner_circle_xform: Transform,

    /// transform of the trace point
    trace_point_xform: Transform, 

    /// transform of the trace line
    /// this is not necessary in terms of pure items that are needed to determine tracker position
    /// but is necessary if you want visuals of all the components.
    /// this stores the transform of the CENTER of the traceline
    trace_line_xform: Transform, 

    /// struct to hold all previous positions
    trace_points: Vec<Vec3>, // could convert to drawing cylinders instead
}

impl HypocycloidTracking {
    fn new(inner_rad: f32, outer_rad:f32, starting_xform: Transform) -> Self {

        // minus x because currently the camera is looking at the origin from behind the x/y plane
        let mut inner_circle_xform = starting_xform;
        inner_circle_xform.translation.x += -1.0 * (outer_rad - inner_rad);

        let mut trace_point_xform = starting_xform;
        trace_point_xform.translation.x += -1.0* outer_rad;

        let mut trace_line_xform = starting_xform;
        trace_line_xform.translation.x += -1.0*  (outer_rad - 0.5*inner_rad);


        HypocycloidTracking{
            outer_circle_xform: starting_xform,
            inner_circle_xform:  inner_circle_xform,
            trace_line_xform: trace_line_xform,
            trace_point_xform: trace_point_xform,

            trace_points: Vec::with_capacity(200_000),
        }
    }
}


pub struct Hypocycloid;
impl Plugin for Hypocycloid {
    fn build(&self, app: &mut App) {

        const START_DELAY: f32 = 2.0;
        const FIXED_INTERVAL:f64 = 0.0005;

        // https://github.com/jakobhellermann/bevy-inspector-egui/tree/v0.27.0
        app.init_resource::<OverrideColor>();
        app.register_type::<OverrideColor>();
        app.add_plugins(ResourceInspectorPlugin::<OverrideColor>::default());

        app.add_systems(Startup, config_gizmo);
        app.add_systems(Startup, Self::setup_scene);
        
        // new system
        app.insert_resource(Time::<Fixed>::from_seconds(FIXED_INTERVAL));
        app.add_systems(FixedUpdate, Self::update_new_tracker.run_if(repeating_after_delay(Duration::from_secs_f32(START_DELAY))));
        app.add_systems(Update, update_fixed_time);
        app.add_systems(Update, Self::draw_new_track);
        app.add_systems(Update, Self:: draw_new_gizmos_and_move_meshes);

        // old system
        // app.add_systems(Update, Self::update_pos2.run_if(repeating_after_delay(Duration::from_secs_f32(START_DELAY))));
        // app.add_systems(Update, Self::draw_track);
        
        // Axes gizmos and gizmo config
        // app.add_systems(Update, Self::draw_gizmos);
        app.add_systems(Update, update_gizmo_config);

    }
}

impl Hypocycloid {

    /// Spawn all the meshes in the right hierarchy
    fn setup_scene(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let outer_circle_radius = hypocycloid_config::OUTER_RAD;
        let torus_tube_radius = 0.1;
        let inner_circle_radius = hypocycloid_config::INNER_RAD;

        commands.insert_resource(Points { points: Vec::with_capacity(200_000)});


        commands.spawn(HypocycloidTracking::new(inner_circle_radius, outer_circle_radius, Transform::from_translation(Vec3::ZERO)));
        
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

    /// Draw gizmos for the meshes
    fn draw_gizmos(
        mut draw: Gizmos,
        // Normally, doing (With<X>, With<Y>, With<Z>) will use an AND. So it will get global transform's that have all 3 components.
        // Using Or means it will get a GlobalTransform IF it has either X, Y, or Z component.
        transforms: Query<&GlobalTransform, Or<(With<TraceLine>, With<TracePoint>, With<RollingCircle>)>>
    ){
        for &transform in transforms.iter() {
            draw.axes(transform, 3.0);
        }
    }

    /// Update all the transforms
    fn update_pos2(
        mut points: ResMut<Points>,
        // rolling_circle: Query<(& mut Transform, &RollingCircle)>, //(With<RollingCircle>, Without<TracePoint>, Without<TraceLine>)>,
        // trace_point: Query<(& mut Transform,&TracePoint)>, //(With<TracePoint>, Without<TraceLine>, Without<RollingCircle>)>,
        // trace_line: Query<(& mut Transform,&TraceLine)>, //(With<TraceLine>, Without<TracePoint>, Without<RollingCircle>)>,
        
        // The children do not at all impact queries. You can query for the children, and use that to get entities from a different query,
        // or you can query for components of the children directly.
        // An alternative to the below solution would be to have 3 different functions each which update one thing.
        // This means you wont need to any the queries being disjoint because all of them would have one query.
        mut set: ParamSet<(
            Query<&mut Transform, With<RollingCircle>>,
            Query<&mut Transform, With<TraceLine>>,
            Query<&GlobalTransform, With<TracePoint>>,
        )>,

        // There is another solution which would be query the children of the parent and then have another query that get all the children components.
        // but in this case, because you have a bunch of single components, 
        // its easier to have separate queries for them as otherwise if you are looping over
        // the children, you need to check the type there somehow so you can do the right transform on it
    )
    {   
        // cant do set.p0.single() directly because it will return a temporary
        let mut param = set.p0();
        let mut circle = param.single_mut();
        circle.rotate_around(Vec3::ZERO,Quat::from_rotation_z(hypocycloid_config::CIRLCE_ROT_RATE));
        // Check if the final rotation is normalized, and if not normalize it
        if ! circle.rotation.is_normalized() {
            circle.rotation = circle.rotation.normalize();
        }

        // repeat the same for the traceline.
        let mut param = set.p1();
        let mut traceline = param.single_mut();
        //  Using y rotation because not in the absolute coordinate frame, it is relative to the parent.
        traceline.rotate_around(Vec3::ZERO,Quat::from_rotation_y(hypocycloid_config::LINE_ROT_RATE));
        if ! traceline.rotation.is_normalized() {
            traceline.rotation = traceline.rotation.normalize();
        }

        let tracepoint = *set.p2().single();
        points.points.push(tracepoint.translation());
    }

    /// Draw the path that the trace point took
    fn draw_track(
        points: Res<Points>,
        color_override: Res<OverrideColor>,
        mut draw : Gizmos,
    ) {
        // Different phases that could be used 
        let a60 = PI/3.0;
        let a120 = PI/3.0*2.0;
        let a180 = PI;

        // Get the points
        let points = &points.points;
        let len_points = points.len();
        // println!("{len_points}");

        for i in 1..len_points {
            // Get the angle and scale down so less repetition
            let angle = (i as f32).to_radians() / 14.0;

            // Check if the color is overridden
            if color_override.overrode {
                draw.line(points[i-1], points[i], color_override.override_color);
                continue;
            }

            // draw the line with the color
            // use desmos to plot the rgb lines with the amp and offset and phase angle to
            // see roughly what your color pattern will look like.
            draw.line(points[i-1], points[i], Color::srgba(
                f32::sin(angle + a60) *0.5+0.5, 
                f32::sin(angle + a180)*0.5 + 0.5, 
                f32::sin(angle)*0.5+0.5,
                1.0
            ));
        }
    }

    /// update the position of the tracer using the new tracking system
    fn update_new_tracker(
        mut tracker: Query<&mut HypocycloidTracking>,

    ){
        let mut tracker = tracker.single_mut();

        // rotate the inner circle and all connected components along the outer circle track
        // this is not rotating the inner circle itself yet
        let hypocycloid_origin = tracker.outer_circle_xform.translation;
        let angle = Quat::from_rotation_z(hypocycloid_config::CIRLCE_ROT_RATE);
        tracker.inner_circle_xform.rotate_around(hypocycloid_origin, angle);
        tracker.trace_point_xform.rotate_around(hypocycloid_origin, angle);
        tracker.trace_line_xform.rotate_around(hypocycloid_origin, angle);

        // rotate the traceline and all connected components
        // I should technically be rotating the inner circle as well but because its a circle
        // it doesnt change anything visually so ignoring for now
        let inner_circle_origin = tracker.inner_circle_xform.translation;
        let angle = Quat::from_rotation_z(hypocycloid_config::LINE_ROT_RATE); 
        tracker.trace_line_xform.rotate_around(inner_circle_origin, angle);
        tracker.trace_point_xform.rotate_around(inner_circle_origin, angle);

        tracker.inner_circle_xform.rotation = tracker.inner_circle_xform.rotation.normalize();
        tracker.trace_point_xform.rotation = tracker.inner_circle_xform.rotation.normalize();
        tracker.trace_line_xform.rotation = tracker.inner_circle_xform.rotation.normalize();

        let final_point_trans = tracker.trace_point_xform.translation;
        tracker.trace_points.push(final_point_trans);


    }

    fn draw_new_gizmos_and_move_meshes(
        mut gizmos: Gizmos,
    ) {
        // you need to undo the parent child hierachy or it wont work.
    }

    /// Draw the new track using the points in `HypocycloidTracking`
    fn draw_new_track(
        tracker: Query<&HypocycloidTracking>,
        color_override: Res<OverrideColor>,
        mut draw : Gizmos,
    ) {
         // Different phases that could be used 
         let a60 = PI/3.0;
         let a120 = PI/3.0*2.0;
         let a180 = PI;
 
         // Get the points
         let points = &tracker.single().trace_points;
         let len_points = points.len();
         if len_points % 1000 == 0 && len_points != 0 {
            println!("len points is {}", len_points);
         }
         // println!("{len_points}");
 
         for i in 1..len_points {
             // Get the angle and scale down so less repetition
             let angle = (i as f32).to_radians() / 14.0;
 
             // Check if the color is overridden
             if color_override.overrode {
                 draw.line(points[i-1], points[i], color_override.override_color);
                 continue;
             }
 
             // draw the line with the color
             // use desmos to plot the rgb lines with the amp and offset and phase angle to
             // see roughly what your color pattern will look like.
             draw.line(points[i-1], points[i], Color::srgba(
                 f32::sin(angle + a60) *0.5+0.5, 
                 f32::sin(angle + a180)*0.5 + 0.5, 
                 f32::sin(angle)*0.5+0.5,
                 1.0
             ));
         }
    }

}

//////////////////////////////////////////
// GIZMO CONFIG CODE
//////////////////////////////////////////

/// Change the initial default settings of a Gizmo
fn config_gizmo(
    mut config_store: ResMut<GizmoConfigStore>,
) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();

    // might improve performance if this number was reduced or switched to a different mode
    config.line_joints = GizmoLineJoint::Miter;
    config.line_perspective = true; // for some reason this makes the line width affect it much much less
    config.line_width = 80.;
    config.depth_bias = -0.2;
}

/// Update the gizmo configuration on the fly
fn update_gizmo_config(
    mut config_store: ResMut<GizmoConfigStore>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();

    let interval:f32 = 200.0;

    if keyboard.pressed(KeyCode::Space) && !keyboard.pressed(KeyCode::ShiftLeft) {
        config.line_width += interval * time.delta_seconds();
    }

    if keyboard.all_pressed([KeyCode::Space, KeyCode::ShiftLeft]){
        config.line_width -= interval * time.delta_seconds();
        if config.line_width < 1.0 {
            config.line_width = 1.0;
        }
    }
}

//////////////////////////////////////////
// TIME CONFIG CODE
//////////////////////////////////////////

fn update_fixed_time(
    mut time: ResMut<Time<Fixed>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let mut interval:f32 = 0.001;

    if keyboard.pressed(KeyCode::ShiftLeft) {
        interval = 0.0001;
    }

    let mut timestep = time.timestep().as_secs_f32();

    // speed up simulation
    if keyboard.pressed(KeyCode::KeyF) {
        timestep = (timestep - interval).clamp(0.0001, 0.5);
        println!("timestep is {}", timestep);
    }

    // slow down simulation
    if keyboard.pressed(KeyCode::KeyR) {
        timestep = (timestep + interval).clamp(0.0001, 0.5);
        println!("timestep is {}", timestep);
    }

    time.set_timestep(Duration::from_secs_f32(timestep));
}

