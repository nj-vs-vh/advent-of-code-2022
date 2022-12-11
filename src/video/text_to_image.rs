use font_kit::canvas::{Canvas, Format, RasterizationOptions};
use font_kit::family_name::FamilyName;
use font_kit::font::Font;
use font_kit::hinting::HintingOptions;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use image;
use num::clamp;
use pathfinder_geometry::transform2d::Transform2F;
use pathfinder_geometry::vector::{Vector2F, Vector2I};
use rand::prelude::*;

lazy_static! {
    static ref FONT: Font = SystemSource::new()
        .select_best_match(&[FamilyName::Monospace], &Properties::new())
        .unwrap()
        .load()
        .unwrap();
}

pub fn text_to_image(
    text: &str,
    target_width_px: u32,
    char_aspect_ratio: f32,
    position_randomization_range_px: f32,
) -> Option<image::GrayImage> {
    if text.len() == 0 {
        return None;
    }
    let lines: Vec<&str> = text.lines().collect();
    let text_width_chars = lines.iter().map(|l| l.len()).max().unwrap() as u32;
    let text_height_chars = lines.len() as u32;

    let char_width_px = target_width_px / text_width_chars;
    let char_height_px = ((char_width_px as f32) / char_aspect_ratio) as u32;

    let width_px = char_width_px * text_width_chars;
    let height_px = char_height_px * text_height_chars;

    let mut imgbuf: image::GrayImage = image::ImageBuffer::new(width_px, height_px);

    let mut rng = rand::thread_rng();
    let mut rand_i32 = || (position_randomization_range_px * (rng.gen::<f32>() - 0.5)) as i32;

    let mut canvas = Canvas::new(
        Vector2I::new(char_width_px as i32, char_height_px as i32),
        Format::A8,
    );

    for (line_idx, line) in lines.iter().enumerate() {
        for (char_idx, ch) in line.chars().enumerate() {
            FONT.rasterize_glyph(
                &mut canvas,
                FONT.glyph_for_char(ch).unwrap(),
                char_height_px as f32,
                Transform2F::from_translation(Vector2F::new(0.0, char_height_px as f32)),
                // Transform2F::from_scale(1.0),
                HintingOptions::None,
                RasterizationOptions::GrayscaleAa,
            )
            .unwrap();

            let random_offset = Vector2I::new(rand_i32(), rand_i32());

            for (i, pixel_value) in canvas.pixels.iter().enumerate() {
                if *pixel_value > 0 {
                    let pixel = imgbuf.get_pixel_mut(
                        clamp(
                            ((char_idx as u32 * char_width_px) + i as u32 % char_width_px) as i32
                                + random_offset.x(),
                            0,
                            width_px as i32 - 1,
                        ) as u32,
                        clamp(
                            ((line_idx as u32 * char_height_px) + i as u32 / char_width_px) as i32
                                + random_offset.y(),
                            0,
                            height_px as i32 - 1,
                        ) as u32,
                    );
                    *pixel = image::Luma([*pixel_value]);
                }
            }
            canvas.pixels = vec![0; canvas.pixels.len()];
        }
    }

    Some(imgbuf)
}

pub fn example() {
    text_to_image(
        "HELLO WORLD\n this is A TEXT\n\n\nmultiline text\n==============================",
        1200,
        1.0,
        0.0,
    )
    .unwrap()
    .save("test.png")
    .unwrap()
}
