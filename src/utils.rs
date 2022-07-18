use anyhow::Result;
use serde::Deserialize;
use std::{fs::read_to_string, path::PathBuf};

#[derive(Deserialize)]
struct RawFormation {
    sodium: bool,
    rotation_info: Vec<RawRotationInfo>,
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
        rotation_info: res,
    })
}
