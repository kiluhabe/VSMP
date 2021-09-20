use crate::vsmp::display::Displayable;
use crate::vsmp::errors::VsmpError;
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::Write;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use uuid::Uuid;
use async_trait::async_trait;

pub struct Ueberzug {
    identifier: String,
    process: Child,
}

trait Formattable {
    fn format(&self) -> Result<String, VsmpError>;
}

#[derive(Serialize, Deserialize, Debug)]
struct UeberzugAddConfig {
    pub action: String,
    pub identifier: String,
    pub x: i32,
    pub y: i32,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synchronously_draw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaler: Option<Scaler>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_position_x: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_position_y: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct UeberzugRemoveConfig {
    pub action: String,
    pub identifier: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
enum Scaler {
    Crop,
    Distort,
    FitContain,
    Contain,
    ForcedCover,
    Cover,
}

impl Ueberzug {
    pub fn default() -> Result<Self, VsmpError> {
        let identifier = Uuid::new_v4().to_hyphenated().to_string();
        let process = Command::new("ueberzug")
            .args(&["layer", "-p", "json"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        Ok(Self {
            identifier,
            process,
        })
    }
    fn command(&self, config: Box<dyn Formattable>) -> Result<(), VsmpError> {
        self.process
            .stdin
            .as_ref()
            .unwrap()
            .write_all(config.format()?.as_bytes())?;
        Ok(())
    }
}

#[async_trait]
impl Displayable for Ueberzug {
    async fn display(&mut self, path: &Path, height: u32, width: u32) -> Result<(), VsmpError> {
        let identifier = &self.identifier;
        self.command(Box::from(UeberzugAddConfig::default(
            identifier.to_string(),
            path.to_string_lossy().to_string(),
            height,
            width,
        )))?;
        Ok(())
    }
}

unsafe impl Sync for Ueberzug {}
unsafe impl Send for Ueberzug {}

impl UeberzugAddConfig {
    pub fn default(identifier: String, path: String, height: u32, width: u32) -> Self {
        Self {
            action: "add".to_string(),
            identifier,
            x: 0,
            y: 0,
            path,
            width: Some(width),
            height: Some(height),
            draw: None,
            synchronously_draw: Some(true),
            scaler: None,
            scaling_position_x: None,
            scaling_position_y: None,
        }
    }
}

impl Formattable for UeberzugAddConfig {
    fn format(&self) -> Result<String, VsmpError> {
        let json = serde_json::to_string(&self)?;
        Ok(format!("{}\n", json))
    }
}

impl UeberzugRemoveConfig {
    #[allow(dead_code)]
    pub fn default(identifier: String) -> Self {
        Self {
            action: "remove".to_string(),
            identifier,
            draw: None,
        }
    }
}

impl Formattable for UeberzugRemoveConfig {
    fn format(&self) -> Result<String, VsmpError> {
        let json = serde_json::to_string(&self)?;
        Ok(format!("{}\n", json))
    }
}

impl Scaler {
    #[allow(dead_code)]
    pub fn value(&self) -> &str {
        match self {
            Scaler::Crop => "crop",
            Scaler::Contain => "contain",
            Scaler::Cover => "contain",
            Scaler::Distort => "distort",
            Scaler::FitContain => "fit_contain",
            Scaler::ForcedCover => "forced_cover",
        }
    }
}
