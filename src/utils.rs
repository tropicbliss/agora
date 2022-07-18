use anyhow::Result;
use serde::Deserialize;
use std::{fs::read_to_string, path::PathBuf};

#[derive(Deserialize)]
struct RawFormation {
    sodium: bool,
    #[serde(default = "x_bounds_default")]
    x_bounds: (i32, i32),
    #[serde(default = "y_bounds_default")]
    y_bounds: (i32, i32),
    #[serde(default = "z_bounds_default")]
    z_bounds: (i32, i32),
    rotation_info: Vec<RawRotationInfo>,
}

fn x_bounds_default() -> (i32, i32) {
    (-10000, 10000)
}

fn y_bounds_default() -> (i32, i32) {
    (10, 60)
}

fn z_bounds_default() -> (i32, i32) {
    (-10000, 10000)
}

#[derive(Deserialize)]
struct RawRotationInfo {
    x: i32,
    y: i32,
    z: i32,
    rotation: i32,
    is_side: bool,
}

pub struct RotationInfo {
    pub info_type: InfoType,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub rotation: i32,
}

#[derive(PartialEq)]
pub enum InfoType {
    TopsAndBottoms,
    Sides,
}

pub struct Formation {
    pub sodium: bool,
    pub x_min: i32,
    pub x_max: i32,
    pub y_min: i32,
    pub y_max: i32,
    pub z_min: i32,
    pub z_max: i32,
    pub rotation_info: Vec<RotationInfo>,
}

pub fn get_config(path: PathBuf) -> Result<Formation> {
    let cfg = read_to_string(path)?;
    let cfg: RawFormation = toml::from_str(&cfg)?;
    let res: Vec<_> = cfg
        .rotation_info
        .into_iter()
        .map(|rot| {
            let (rotation, info_type) = if rot.is_side {
                (rot.rotation % 2, InfoType::Sides)
            } else {
                (rot.rotation, InfoType::TopsAndBottoms)
            };
            RotationInfo {
                x: rot.x,
                y: rot.y,
                z: rot.z,
                rotation,
                info_type,
            }
        })
        .collect();
    Ok(Formation {
        sodium: cfg.sodium,
        x_min: cfg.x_bounds.0,
        x_max: cfg.x_bounds.1,
        y_min: cfg.y_bounds.0,
        y_max: cfg.y_bounds.1,
        z_min: cfg.z_bounds.0,
        z_max: cfg.z_bounds.1,
        rotation_info: res,
    })
}
