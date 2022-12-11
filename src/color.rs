use colors_transform::{Color, Hsl};
use image::Rgb;

pub fn get_rgb_pixel(hue: u16, sat: u8, lght: u8) -> Rgb<u8> {
    let c_hsl = Hsl::from(hue as f32, sat as f32, lght as f32);
    let c_rgb = c_hsl.to_rgb();
    let (r, g, b) = c_rgb.as_tuple();
    Rgb([r as u8, g as u8, b as u8])
}
