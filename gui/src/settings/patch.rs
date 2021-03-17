use std::fs::File;
use std::io::prelude::*;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct PatchFileSettings {
  pub original_rom: String,
  pub modified_rom: String,
  pub asar: String,
  pub patch: String,
}

impl PatchFileSettings {
  pub fn new(filename: &str) -> Self {
    match Self::_new(filename) {
      Ok(patch) => patch,
      Err(error) => {
        if error.kind() != std::io::ErrorKind::NotFound {
          eprintln!("Error while reading patch file settings: {}", error);
        }
        Self::default()
      }
    }
  }

  pub fn _new(filename: &str) -> std::io::Result<Self> {
    let mut file = File::open(filename)?;
    let mut contents = Vec::<u8>::new();
    file.read_to_end(&mut contents)?;

    match bincode::deserialize(&contents) {
      Ok(patch) => Ok(patch),
      Err(error) => {
        eprintln!("Error while reading patch file settings: {}", error);
        Ok(Self::default())
      }
    }
  }
}

