use std::fs::File;
use std::io::{self, prelude::*};

use image::GrayImage;
use rav1e::config::SpeedSettings;
use rav1e::*;

pub fn write_video(
    images: impl Iterator<Item = GrayImage>,
    width: usize,
    height: usize,
    filename: &str,
) {
    let encoder_config = EncoderConfig {
        width,
        height,
        speed_settings: SpeedSettings::from_preset(8),
        ..Default::default()
    };
    let config = Config::new().with_encoder_config(encoder_config.clone());
    let mut ctx: Context<u8> = config.new_context().unwrap();

    for image in images {
        print!(".");
        io::stdout().flush().unwrap();

        let mut f = ctx.new_frame();
        let gray_plane = &mut f.planes[0];
        let stride = (encoder_config.width + gray_plane.cfg.xdec) >> gray_plane.cfg.xdec;
        let pixels: Vec<u8> = image.pixels().map(|p| p.0[0]).collect();
        gray_plane.copy_from_raw_u8(pixels.as_slice(), stride, 1);

        match ctx.send_frame(f.clone()) {
            Ok(_) => {}
            Err(e) => {
                println!("Error sending frame: {:?}", e);
                break;
            }
        }
    }

    ctx.flush();

    println!("\nVideo preprocessing done, encoding...");

    let mut file = File::create(filename).expect("Can't create file");
    let mut written_bytes = 0;
    loop {
        match ctx.receive_packet() {
            Ok(pkt) => {
                file.write(pkt.data.as_slice()).unwrap();
                written_bytes += pkt.data.len();
                println!("Written {} bytes", written_bytes);
            }
            Err(e) => match e {
                EncoderStatus::LimitReached => {
                    println!("Video written");
                    break;
                }
                EncoderStatus::Encoded => {}
                EncoderStatus::NeedMoreData => {
                    break;
                }
                _ => {
                    panic!("Unable to receive packet");
                }
            },
        }
    }
}
