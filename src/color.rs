use crate::vec3::Color;

pub fn paint(mut pixel_color: Color, samples: usize) -> String {
    let scale = 1.0 / samples as f64;
    pixel_color *= scale;
    pixel_color.sqrt();
    pixel_color.clamp();
    format!("{}\n", pixel_color)
}
