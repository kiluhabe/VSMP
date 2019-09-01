extern crate rppal;
extern crate image;

mod epd_interface;
mod errors;
mod epd7in5;
mod pin;
mod command;

use rppal::spi::{Spi, Mode, Bus, SlaveSelect};
use rppal::gpio::Gpio;
use image::FilterType;
use std::path::Path;
use std::{thread, time};

use epd_interface::EPDInterface;
use epd7in5::EPD;
use errors::VSMPError;

fn init_epd(height: u32, width: u32) -> EPD {
    let gpio = Gpio::new().unwrap();
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 2000000, Mode::Mode0).unwrap();

    let epd_interface = EPDInterface {
        gpio: gpio, spi: spi
    };
    return EPD {
        height: height,
        width: width,
        interface: epd_interface
    };
}

fn get_image_buffer(path_str: &str, width: u32, height: u32) -> Result<Vec<u8>, VSMPError> {
    let image_path = Path::new(path_str);
    let img = image::open(&image_path)?;
    let resized_image = img.resize(
        width / 2, height, FilterType::Lanczos3);
    let buffer = resized_image
        .grayscale()
        .adjust_contrast(50.0)
        .to_luma()
        .to_vec()
        .into_iter().map(|e| {
            if e == 255 { 0x03 } else { e }
        })
        .collect();
    Ok(buffer)
}

fn blank(width: usize, height: usize) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(width * height /2);
    for _ in 1..(width * height / 2) {
        buffer.push(0x03);
    };
    return buffer
}

fn main() -> Result<(), VSMPError> {
    let mut epd = init_epd(384, 640);
    let buffer = get_image_buffer(
        "/tmp/vsmp/images/sample.png", epd.width, epd.height)?;

    println!("{}", "initting...");
    epd.init()?;
    thread::sleep(time::Duration::from_millis(200));
    println!("{}", "rendering...");
    epd.display_frame(&buffer)?;
    println!("{}", "done.");

    Ok(())
}
