extern crate rppal;
extern crate image;

mod epd_interface;
mod errors;
mod epd7in5;

use rppal::spi::{Spi, Mode, Bus, SlaveSelect};
use rppal::gpio::Gpio;
use image::{FilterType, GrayImage};
use std::path::Path;

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

fn main() -> Result<(), VSMPError> {
    let mut epd = init_epd(384, 640);

    let image_path = Path::new("/tmp/vsmp/images/sample.bmp");
    let img = image::open(&image_path)?;
    let resized_image = img.resize(epd.width, epd.height, FilterType::Lanczos3);
    let buffer = epd.get_frame_buffer(resized_image.as_luma8().unwrap());
    epd.init()?;
    epd.display_frame(&buffer)?;

    Ok(())
}
