use image::GrayImage;

pub mod text_to_image;
pub mod writer;

pub fn texts_to_video(strings: Vec<&str>) {
    let mut images: Vec<GrayImage> = Vec::new();

    for _ in 0..300 {
        images.push(
            text_to_image::text_to_image(
                "HELLO WORLD\ni'm producing the video\n\n\nwoooooooooo",
                800,
                1.0,
                3.0,
            )
            .unwrap(),
        )
    }

    let width = images.iter().map(|i| i.width()).max().unwrap();
    let height = images.iter().map(|i| i.height()).max().unwrap();
    writer::write_video(
        images.into_iter(),
        width as usize,
        height as usize,
        "test.av1",
    )
}
