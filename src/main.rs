extern crate rppal;

mod epd_interface;
mod errors;
mod epd7in5;

use rppal::spi::{Spi, Mode, Bus, SlaveSelect};
use rppal::gpio::Gpio;
use image::{ImageBuffer, Luma};

use epd_interface::EPDInterface;
use epd7in5::EPD;
use errors::VSMPError;

fn main() -> Result<(), VSMPError>{
    let gpio = Gpio::new().unwrap();
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 2000000, Mode::Mode0).unwrap();

    let epd_interface = EPDInterface {
        gpio: gpio, spi: spi
    };
    let mut epd = EPD {
        height: 384,
        width: 640,
        interface: epd_interface
    };

    let img = ImageBuffer::from_fn(640, 384, |x, y| {
        if x % 2 == 0 {
            Luma([0u8])
        } else {
            Luma([255u8])
        }
    });
    let buffer = epd.get_frame_buffer(img);

    epd.init()?;
    epd.display_frame(&buffer)?;

    epd.interface.sleep_ms(10000);

    Ok(())
}
