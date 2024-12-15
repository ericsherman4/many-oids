
/// Scene settings
pub mod colors_config {
    use bevy::prelude::Color;
    use bevy::prelude::Srgba;

    pub const BG_COLOR: &str = "000000";

    pub const BLUE: Color = Color::Srgba(Srgba::BLUE);
    pub const GREEN: Color = Color::Srgba(Srgba::GREEN);
    pub const RED: Color = Color::Srgba(Srgba::RED);

    /// Get a color from a hex string
    pub fn get_color(hex: &str) -> Color {
        Srgba::hex(hex).unwrap().into()
    }
}

pub mod lights_config {
    use bevy::math::Vec3;

    pub const POS_COMPONENT: f32 = 10.0;
    pub const POS: Vec3 = Vec3::splat(POS_COMPONENT);
    pub const POS_2: Vec3 = Vec3::new(-7.2, -10.4, -8.8);
}


// TODO: you should make the draw axes stuff into a a plugin
pub mod axis_config {
    pub const GIRTH: f32 = 0.05;
    pub const LENGTH: f32 = 10.;
    pub const HALF_LENGTH: f32 = LENGTH / 2.;
    pub const ORIGIN_SPHERE_RADIUS: f32 = GIRTH;
}

pub mod cam_config {
    use bevy::math::Vec3;
    pub const POS: Vec3 = Vec3::splat(7.0);
}