use std::{f32::consts::PI, time::Duration};
use bevy::{prelude::*, render::mesh::TorusMeshBuilder, time::common_conditions::repeating_after_delay};
use crate::config::{colors_config, hypocycloid_config};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

#[derive(Reflect)]
struct ColorControl {
    override_enabled: bool,
    override_color: Color,
}

impl Default for ColorControl{
    fn default() -> Self {
        Self {
            override_enabled: false,
            override_color: Color::WHITE,
        }
    }
}

/// A resource that will override the color of the hypocycloid 
#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
struct HypocycloidControls {
    color : ColorControl,
    inferior_color : ColorControl,
    superior_color : ColorControl,
    draw_inferior: bool,
    draw_exterior: bool,
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

    /// distance from the center of the trace point to the inferior Hypotrchoid
    /// This value should be negative as it should lie inside the circle.
    inferior_radius: f32,

    /// distance from the center of the trace point to the superior Hypotrchoid
    /// This value should be positive as it should lie outside the circle
    superior_radius: f32,

    /// struct to hold all previous positions
    trace_points: Vec<Vec3>, // could convert to drawing cylinders instead

    /// struct to hold the forward axis of the trace line so that you can 
    /// offset to get interior and exterior positions
    /// slightly more memory efficient then storing hypo pos, inferior pos, and superior pos separately.
    trace_point_forward_vecs:  Vec<Vec3>,
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
            inferior_radius: hypocycloid_config::INFERIOR_RAD,
            superior_radius: hypocycloid_config::SUPERIOR_RAD,

            trace_points: Vec::with_capacity(200_000),
            trace_point_forward_vecs: Vec::with_capacity(200_000),
        }
    }
}

// impl Default for HypocycloidTracking {
//     fn default() -> Self {
//         let inner_rad = 5.0;
//         let outer_rad = inner_rad * 2.1;
//         let starting_xform = Transform::from_translation(Vec3::ZERO);

//         // minus x because currently the camera is looking at the origin from behind the x/y plane
//         let mut inner_circle_xform = starting_xform;
//         inner_circle_xform.translation.x += -1.0 * (outer_rad - inner_rad);

//         let mut trace_point_xform = starting_xform;
//         trace_point_xform.translation.x += -1.0* outer_rad;

//         let mut trace_line_xform = starting_xform;
//         trace_line_xform.translation.x += -1.0*  (outer_rad - 0.5*inner_rad);

//         HypocycloidTracking{
//             outer_circle_xform: starting_xform,
//             inner_circle_xform: inner_circle_xform,
//             trace_line_xform: trace_line_xform,
//             trace_point_xform: trace_point_xform,
//             inferior_radius: inner_rad - 1.0,
//             superior_radius: inner_rad + 1.0,

//             trace_points: Vec::with_capacity(200_000),
//         }
        
//     }
// }


pub struct Hypocycloid;
impl Plugin for Hypocycloid {
    fn build(&self, app: &mut App) {

        const START_DELAY: f32 = 2.0;
        const FIXED_INTERVAL:f64 = 0.01;

        // https://github.com/jakobhellermann/bevy-inspector-egui/tree/v0.27.0
        app.init_resource::<HypocycloidControls>();
        app.register_type::<HypocycloidControls>();
        app.add_plugins(ResourceInspectorPlugin::<HypocycloidControls>::default());

        // setup
        app.add_systems(Startup, config_gizmo);
        app.add_systems(Startup, Self::setup_scene);
        
        // new system
        app.insert_resource(Time::<Fixed>::from_seconds(FIXED_INTERVAL));
        app.add_systems(FixedUpdate, Self::update_new_tracker.run_if(repeating_after_delay(Duration::from_secs_f32(START_DELAY))));
        app.add_systems(Update, update_fixed_time);
        app.add_systems(Update, Self::draw_new_track);
        // app.add_systems(Update, Self::draw_superior);
        app.add_systems(Update, Self::draw_interior);
        app.add_systems(Update, Self:: update_mesh_pos);
        app.add_systems(Update, Self::hide_meshes);
        
        // Axes gizmos and gizmo config
        app.add_systems(Update, Self::draw_gizmos);
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
        let inner_circle_radius = hypocycloid_config::INNER_RAD;
        let mesh_thickness = 0.5;


        let hypocycloid = HypocycloidTracking::new(inner_circle_radius, outer_circle_radius, Transform::from_translation(Vec3::ZERO).with_rotation(Quat::from_rotation_x(PI/2.0)));

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(
                    TorusMeshBuilder {
                        torus: Torus{
                            major_radius: outer_circle_radius,
                            minor_radius: mesh_thickness,
                        },
                        major_resolution: 100,
                        ..default()
                    }
                ),
                material: materials.add(Color::WHITE),
                transform : hypocycloid.outer_circle_xform,
                visibility: Visibility::Visible,
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
                            minor_radius: mesh_thickness
                        },
                        major_resolution: 100,
                        ..default()
                    }
                ),
                material: materials.add(Color::srgb(1.0, 0.0, 0.0)),
                // because of this rotation, you are rotating the coordinate frame of all the children.
                // so now instead of z pointing in /out of the screen, y does. 
                transform: hypocycloid.inner_circle_xform,
                ..default()
            },
            RollingCircle
        ));

        commands.spawn((
            PbrBundle {
                mesh:  meshes.add(Cuboid::from_size(Vec3::new(inner_circle_radius,mesh_thickness,mesh_thickness))),
                material: materials.add(Color::WHITE),
                transform :  hypocycloid.trace_line_xform,
                ..default()
            }, 
            TraceLine
        ));

        commands.spawn((
            PbrBundle {
                visibility: Visibility::Visible,
                mesh : meshes.add(Sphere {radius:mesh_thickness*1.2, ..default()}),             
                transform: hypocycloid.trace_point_xform,
                ..default()
            },
            TracePoint
        ));

        commands.spawn(hypocycloid);
        // commands.spawn(HypocycloidTracking::new(inner_circle_radius,outer_circle_radius, Transform::from_translation(Vec3::Z*10.0). with_rotation(Quat::from_rotation_x(PI/2.0))));
        // commands.spawn(HypocycloidTracking::new(inner_circle_radius,outer_circle_radius, Transform::from_translation(-Vec3::Z*10.0). with_rotation(Quat::from_rotation_x(PI/2.0))));

    }

    /// Draw gizmos for the meshes
    fn draw_gizmos(
        mut draw: Gizmos,
        // Normally, doing (With<X>, With<Y>, With<Z>) will use an AND. So it will get global transform's that have all 3 components.
        // Using Or means it will get a GlobalTransform IF it has either X, Y, or Z component.
        transforms: Query<&GlobalTransform, Or<(With<TraceLine>, With<TracePoint>, With<RollingCircle>)>>,
        
    ){
        for &transform in transforms.iter() {
            draw.axes(transform, 3.0);
        }
    }

    /// update the position of the tracer using the new tracking system
    fn update_new_tracker(
        mut tracker_query: Query<&mut HypocycloidTracking>,

    ){
        for mut tracker in tracker_query.iter_mut() {
            // rotate the inner circle and all connected components along the outer circle track
            // this is not rotating the inner circle itself yet
            let hypocycloid_origin = tracker.outer_circle_xform.translation;
            let angle = Quat::from_axis_angle(tracker.inner_circle_xform.local_y().into(), hypocycloid_config::CIRLCE_ROT_RATE);
            tracker.inner_circle_xform.rotate_around(hypocycloid_origin, angle);
            tracker.trace_point_xform.rotate_around(hypocycloid_origin, angle);
            tracker.trace_line_xform.rotate_around(hypocycloid_origin, angle);
    
            // rotate the traceline and all connected components
            // TODO: I should technically be rotating the inner circle as well but because its a circle
            // it doesnt change anything visually so ignoring for now
            let inner_circle_origin = tracker.inner_circle_xform.translation;
            let angle = Quat::from_axis_angle(tracker.inner_circle_xform.local_y().into(),hypocycloid_config::LINE_ROT_RATE); 
            tracker.trace_line_xform.rotate_around(inner_circle_origin, angle);
            tracker.trace_point_xform.rotate_around(inner_circle_origin, angle);
    
            tracker.inner_circle_xform.rotation = tracker.inner_circle_xform.rotation.normalize();
            tracker.trace_point_xform.rotation = tracker.trace_point_xform.rotation.normalize();
            tracker.trace_line_xform.rotation = tracker.trace_line_xform.rotation.normalize();
    
            let final_point_trans = tracker.trace_point_xform.translation;
            tracker.trace_points.push(final_point_trans);
            
            let final_forward_vec: Vec3 = tracker.trace_line_xform.local_x().into();
            tracker.trace_point_forward_vecs.push(final_forward_vec);
        }
    }

    /// Draw the new track using the points in `HypocycloidTracking`
    fn draw_new_track(
        tracker_query: Query<&HypocycloidTracking>,
        controls: Res<HypocycloidControls>,
        mut draw : Gizmos,
    ) {
         // Different phases that could be used 
         let a60 = PI/3.0;
         let a120 = PI/3.0*2.0;
         let a180 = PI;

         for tracker in tracker_query.iter() {
             // Get the points
            let points = &tracker.trace_points;
            let len_points = points.len();
            if len_points % 1000 == 0 && len_points != 0 {
                println!("len points is {}", len_points);
            }
            // println!("{len_points}");
             for i in 1..len_points {
                 // Get the angle and scale down so less repetition
                 let angle = (i as f32).to_radians() / 14.0;
     
                 // Check if the color is overridden
                 if controls.color.override_enabled {
                     draw.line(points[i-1], points[i], controls.color.override_color);
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

    fn draw_path_offset(
        tracker: &HypocycloidTracking,
        offset_radius: f32, 
        color_control: &ColorControl,
        draw: &mut Gizmos,
    ) {
        // know its local x based on the params used when building it
        let points = &tracker.trace_points;
        let forward_vecs = &tracker.trace_point_forward_vecs;

        // both vectors must be of the same length
        assert_eq!(points.len(), forward_vecs.len());

        if points.len() == 0 {
            return;
        }

        let mut previous = points[0] + forward_vecs[0]*offset_radius;
        let mut current: Vec3;

        for i in 1..points.len() {
            // Check if the color is overridden
            current = points[i] + forward_vecs[i]*offset_radius;

            if color_control.override_enabled {
                draw.line(previous, current , color_control.override_color);
            }
            else {
                draw.line(previous, current , colors_config::get_color("00FF00"));

            }
            previous = current;
        }
    }

    fn draw_interior(
        tracker_query: Query<&HypocycloidTracking>,
        controls: Res<HypocycloidControls>,
        mut draw : Gizmos,
    ) {
        for tracker in tracker_query.iter() {
            Self::draw_path_offset(
                tracker,
                tracker.inferior_radius, 
                &controls.inferior_color, 
                &mut draw
            );
        }
    }

    fn hide_meshes(
        keyboard: Res<ButtonInput<KeyCode>>,
        mut vis_query: Query<&mut Visibility, Or<(With<RollingCircle>, With<TracePoint>,With<TraceLine>,With<OuterCircle>)>>,
    ) 
    {
        if keyboard.just_pressed(KeyCode::KeyH) {
            println!("changing vis");
            for mut vis in vis_query.iter_mut() {
                if *vis == Visibility::Hidden {
                    *vis = Visibility::Visible;
                    println!("making vis");
                }
                else {
                    *vis = Visibility::Hidden;
                    println!("making hidden");
                }
            }
        }
    }

    fn update_mesh_pos(
        // rolling_circle: Query<(& mut Transform, &RollingCircle)>, //(With<RollingCircle>, Without<TracePoint>, Without<TraceLine>)>,
        // trace_point: Query<(& mut Transform,&TracePoint)>, //(With<TracePoint>, Without<TraceLine>, Without<RollingCircle>)>,
        // trace_line: Query<(& mut Transform,&TraceLine)>, //(With<TraceLine>, Without<TracePoint>, Without<RollingCircle>)>,
        // replace these with param sets! way too annoying to create disjointed queries

        //another option is Query<&mut Transform, Or<(With<RollingCircle>, With<TracePoint>,With<TraceLine>,With<OuterCircle>)>>,
        // which will get all of the transforms
        
        // An alternative to the below solution would be to have 3 different functions each which update one thing.
        // This means you wont need to any the queries being disjoint because all of them would have one query.
        mut set: ParamSet<(
            Query<&mut Transform, With<RollingCircle>>,
            Query<&mut Transform, With<TracePoint>>,
            Query<&mut Transform, With<TraceLine>>,
            Query<&mut Transform, With<OuterCircle>>
        )>,
        tracker_query: Query<&HypocycloidTracking>,        
    ){
        for tracker in tracker_query.iter() {
            let mut param = set.p0();
            let mut obj = param.single_mut();
            *obj = tracker.inner_circle_xform;
    
            let mut param = set.p1();
            let mut obj = param.single_mut();
            *obj = tracker.trace_point_xform;
    
            let mut param = set.p2();
            let mut obj = param.single_mut();
            *obj = tracker.trace_line_xform;
    
            let mut param = set.p3();
            let mut obj = param.single_mut();
            *obj = tracker.outer_circle_xform;
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

