/// Scene settings
pub mod colors_config {
    use bevy::prelude::Color;
    use bevy::prelude::Srgba;

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

    pub const BG_COLOR: &str = "000000";
    pub const GIZMOS_ON: bool = false;

    // Light 1
    pub const POS_1: Vec3 = Vec3::splat(7.0);
    pub const LOOKING_AT_1: Vec3 = Vec3::ZERO;

    //Light 2
    pub const POS_2: Vec3 = Vec3::splat(-7.0);
    pub const LOOKING_AT_2: Vec3 = Vec3::ZERO;
}

pub mod origin_config {
    use bevy::prelude::Color;
    use bevy::prelude::Srgba;

    pub const AXIS_GIRTH: f32 = 0.05;
    pub const AXIS_LENGTH: f32 = 10.;
    pub const ORIGIN_SPHERE_RADIUS: f32 = AXIS_GIRTH;

    pub const COLOR_Z: Color = Color::Srgba(Srgba::BLUE);
    pub const COLOR_Y: Color = Color::Srgba(Srgba::GREEN);
    pub const COLOR_X: Color = Color::Srgba(Srgba::RED);
}

pub mod cam_config {
    use bevy::math::Vec3;
    pub const POS: Vec3 = Vec3::splat(7.0);
    pub const LOOKING_AT: Vec3 = Vec3::ZERO;
}
