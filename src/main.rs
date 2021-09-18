extern crate clap;
extern crate ctrlc;
extern crate dirs;
extern crate image;
extern crate rppal;
extern crate serde;
extern crate serde_json;
extern crate uuid;

mod vsmp;

use clap::Clap;
use std::sync::{Arc, Mutex};
use vsmp::cli::Options;
use vsmp::errors::VsmpError;
use vsmp::Vsmp;

fn main() -> Result<(), VsmpError> {
    let options: Options = Options::parse();
    let config = options.to_config();
    let vsmp = Arc::from(Mutex::from(Vsmp::new(config)?));
    let cleaner = vsmp.clone();

    ctrlc::set_handler(move || {
        let cleaner = cleaner.lock().unwrap();
        cleaner.cleanup().unwrap();
    })?;

    let mut player = vsmp.lock().unwrap();
    player.play(config)?;
    Ok(())
}
