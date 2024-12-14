
/// Scene settings
pub mod scene {
    use bevy::prelude::Color;
    use bevy::prelude::Srgba;

    pub const BG_COLOR: &str = "000000";

    /// Get a color from a hex string
    pub fn get_color(hex: &str) -> Color {
        Srgba::hex(hex).unwrap().into()
    }
}