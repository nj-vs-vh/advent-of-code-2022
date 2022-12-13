use std::collections::HashMap;

use font_kit::canvas::{Canvas, Format, RasterizationOptions};
use font_kit::family_name::FamilyName;
use font_kit::font::Font;
use font_kit::hinting::HintingOptions;
use font_kit::properties::{Properties, Weight};
use font_kit::source::SystemSource;
use image;
use num::clamp;
use pathfinder_geometry::transform2d::Transform2F;
use pathfinder_geometry::vector::{Vector2F, Vector2I};
use rand::prelude::*;

use crate::utils::ascii_box;
use crate::utils::repeated_char;
use crate::visualizer::CharVisualizationOption;

fn get_font(is_bold: bool) -> Font {
    let mut properties = Properties::new();
    if is_bold {
        properties.weight = Weight::BOLD;
    }
    SystemSource::new()
        .select_best_match(&[FamilyName::Monospace], &properties)
        .unwrap()
        .load()
        .unwrap()
}

pub struct CharMatrix {
    lines: Vec<String>,
}

impl CharMatrix {
    pub fn new(text: &str) -> CharMatrix {
        CharMatrix {
            lines: text.lines().map(|l| l.to_string()).collect(),
        }
    }

    pub fn dimensions(&self) -> (usize, usize) {
        let maybe_width = self.lines.iter().map(|l| (**l).chars().count()).max();
        if let Some(width) = maybe_width {
            (width, self.lines.len())
        } else {
            (0, 0)
        }
    }

    pub fn ensure_dimensions(&mut self, width: usize, height: usize) {
        let mut new_lines: Vec<String> = Vec::new();
        for line in self.lines.iter().take(height) {
            let current_width = line.chars().count();
            if current_width > width {
                new_lines.push(line[0..width].to_string());
            } else {
                let mut extended_line = line.to_string();
                extended_line.extend(repeated_char(' ', width - current_width).chars());
                new_lines.push(extended_line);
            }
        }

        if self.lines.len() < height {
            for _ in 0..(height - self.lines.len()) {
                new_lines.push("".to_string());
            }
        }

        self.lines = new_lines;
    }
}

pub fn text_to_image(
    char_matrix: &CharMatrix,
    target_width_px: u32,
    char_aspect_ratio: f32,
    position_randomization_range_px: f32,
    char_opts: &Vec<CharVisualizationOption>,
) -> Option<image::RgbaImage> {
    let (text_width_chars_, text_height_chars_) = char_matrix.dimensions();
    if text_width_chars_ == 0 {
        return None;
    }

    let mut font_by_char: HashMap<char, Font> = HashMap::new();
    for char_opt in char_opts {
        if char_opt.is_bold {
            font_by_char.insert(char_opt.char, get_font(true));
        }
    }
    let default_font = get_font(false);

    let text_width_chars = text_width_chars_ as u32;
    let text_height_chars = text_height_chars_ as u32;

    let char_width_px = target_width_px / text_width_chars;
    let char_height_px = ((char_width_px as f32) / char_aspect_ratio) as u32;

    let width_px = char_width_px * text_width_chars;
    let height_px = char_height_px * text_height_chars;

    let mut imgbuf: image::RgbaImage =
        image::ImageBuffer::from_pixel(width_px, height_px, image::Rgba([0, 0, 0, 255]));

    let mut rng = rand::thread_rng();
    let mut rand_i32 = || (position_randomization_range_px * (rng.gen::<f32>() - 0.5)) as i32;

    let canvas_height = char_height_px * 2; // to draw underline elemenets like for p and q
    let mut canvas = Canvas::new(
        Vector2I::new(char_width_px as i32, canvas_height as i32),
        Format::A8,
    );

    for (line_idx, line) in char_matrix.lines.iter().enumerate() {
        for (char_idx, ch) in line.chars().enumerate() {
            let font = font_by_char.get(&ch).unwrap_or(&default_font);
            let maybe_char_visualization_opt = char_opts.iter().find(|o| o.char == ch);

            font.rasterize_glyph(
                &mut canvas,
                font.glyph_for_char(ch).unwrap(),
                char_height_px as f32 * 1.1,
                Transform2F::from_translation(Vector2F::new(0.0, char_height_px as f32)),
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
                                + random_offset.x()
                                + (char_height_px as i32 / 5), // manual magic offset, don't want to figure this out
                            0,
                            width_px as i32 - 1,
                        ) as u32,
                        clamp(
                            ((line_idx as u32 * char_height_px) + i as u32 / char_width_px) as i32
                                + random_offset.y()
                                - (char_width_px as i32 / 5),
                            0,
                            height_px as i32 - 1,
                        ) as u32,
                    );
                    *pixel = if let Some(opt) = maybe_char_visualization_opt {
                        let dimming = *pixel_value as u32;
                        image::Rgba([
                            (dimming * opt.color[0] as u32 / 255) as u8,
                            (dimming * opt.color[1] as u32 / 255) as u8,
                            (dimming * opt.color[2] as u32 / 255) as u8,
                            255,
                        ])
                    } else {
                        image::Rgba([*pixel_value, *pixel_value, *pixel_value, 255])
                    }
                }
            }
            canvas.pixels = vec![0; canvas.pixels.len()];
        }
    }

    Some(imgbuf)
}

#[allow(dead_code)]
pub fn example() {
    let content = "HELLO WORLD\n this is A TEXT\n\nwith q and p\nmultiline text\n==============================".to_string();
    let content_preproc = ascii_box(content, 1, 0);
    text_to_image(
        &CharMatrix::new(&content_preproc),
        800,
        1.0,
        1.0,
        &Vec::new(),
    )
    .unwrap()
    .save("test.png")
    .unwrap()
}
