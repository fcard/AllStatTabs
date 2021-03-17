mod file;
mod tabs;
mod expgoldtech;
mod patch;

use std::fs::File;
use std::io::prelude::*;
use std::io;
use file::*;
use tabs::*;
use expgoldtech::*;
use patch::*;

#[derive(Clone)]
pub struct HackSettings {
  pub always_save : bool,
  pub tabs : TabsSettings,
  pub expgoldtech : ExpGoldTechSettings,
  pub patch : PatchFileSettings
}

impl Default for HackSettings {
  fn default() -> Self {
    Self {
      always_save: true,
      tabs: TabsSettings::new(&KeyValuePairs::new()),
      expgoldtech: ExpGoldTechSettings::new(&KeyValuePairs::new()),
      patch: PatchFileSettings::default(),
    }
  }
}

impl HackSettings {
  pub fn new() -> Self {
    let settingsgui = HackSettings::settings_keyvalues(".settingsgui.conf");
    let settings = HackSettings::settings_keyvalues("settings.conf");
    let tabs = TabsSettings::new(&settings);
    let expgoldtech = ExpGoldTechSettings::new(&settings);
    let patch = PatchFileSettings::new(".settingsgui.patch.conf");
    Self {
      always_save: settingsgui.get_bool("AlwaysSave", true),
      tabs,
      expgoldtech,
      patch,
    }
  }

  pub fn settings_keyvalues(filename: &str) -> KeyValuePairs {
    match KeyValuePairs::from_file(filename) {
      Ok(settings) => {
        return settings;
      }
      Err(error) => {
        if error.kind() != io::ErrorKind::NotFound {
          eprintln!("Error while trying to read '{}': {}", filename, error);
        }
        KeyValuePairs::new()
      }
    }
  }

  pub fn save(&self) {
    if let Err(error) = self._save() {
      eprintln!("Error while trying to write to save settings: {}", error);
    }
  }

  pub fn _save(&self) -> io::Result<()> {
    let mut file = File::create(".settingsgui.patch.conf")?;
    if let Ok(patch) = bincode::serialize(&self.patch) {
      file.write_all(&patch)?;
    }

    let mut file = File::create(".settingsgui.conf")?;
    writeln!(file,"AlwaysSave = {}", self.always_save as i8)?;

    let mut file = File::create("settings.conf")?;
    writeln!(file,"BetterTabs = {}", self.tabs.better_tabs as i8)?;
    writeln!(file,"PowerTabIncreasesHit = {}", self.tabs.power_tab_increases_hit as i8)?;
    writeln!(file,"JetsOfTime = {}", self.tabs.jets_of_time_rando as i8)?;
    writeln!(file,"AllowStatDecrease = {}", self.tabs.allow_stat_decrease as i8)?;
    writeln!(file,"PowerIncrease = {}", self.tabs.power_increase)?;
    writeln!(file,"StaminaIncrease = {}", self.tabs.stamina_increase)?;
    writeln!(file,"SpeedIncrease = {}", self.tabs.speed_increase)?;
    writeln!(file,"MagicIncrease = {}", self.tabs.magic_increase)?;
    writeln!(file,"HitIncrease = {}", self.tabs.hit_increase)?;
    writeln!(file,"EvadeIncrease = {}", self.tabs.evade_increase)?;
    writeln!(file,"MagicDefenseIncrease = {}", self.tabs.magic_defense_increase)?;
    writeln!(file,"IncreaseExpGoldTech = {}", self.expgoldtech.allow as i8)?;
    writeln!(file,"GradualExpIncrease = {}", self.expgoldtech.gradual_exp as i8)?;
    writeln!(file,"GradualExpMin = {}", self.expgoldtech.gradual_exp_min)?;
    Ok(())
  }
}

