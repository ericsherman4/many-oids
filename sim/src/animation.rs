use bevy::prelude::*;

use crate::config::colors_config;

pub struct HypocycloidTest;
impl Plugin for HypocycloidTest {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::spawn_self);
    }
}

impl HypocycloidTest {

    /// Spawn the components of the hypocycloid
    fn spawn_self(
        // mut commands:Commands,
        // mut meshes: ResMut<Assets<Mesh>>,
        // mut materials: ResMut<Assets<StandardMaterial>>,
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

        // this doesnt work as i thought it would, only draws straight lines?
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
    }

    fn super_vec(i: u32, offset: u32) -> Vec3
    {
        let i = i as f32;
        Vec3::new(i*i + offset as f32, i*i*i, i*i*i*i)
    }
}






// https://wwwtyro.net/2019/11/18/instanced-lines.html
// https://www.reddit.com/r/bevy/comments/1ciwzb1/is_it_bad_to_use_gizmos_in_the_game/
// https://github.com/ForesightMiningSoftwareCorporation/bevy_polyline
// https://www.reddit.com/r/bevy/comments/1e04xk8/how_to_create_2d_object_from_arbitrary_list_of/
// seems like you either use polyline or gizmos. giszmos seems performant enough even tho they are redrawn every frame

// method one seems to be try gizmos? 
// if that doesnt work, try primitives https://docs.rs/bevy/0.14.2/bevy/math/primitives/index.html
// if that doesnt work try polyline
// if that doesnt work, try custom meshes with maybe a custom shader

