use crate::gui::*;

use std::process::exit;
use std::process::Command;
use std::fs::copy;
use std::path::Path;


fn string_setting<B: ByteSetting>(
    bs: &mut B,
    settings: &mut HackSettings,
    text: String, min: i8, max: i8
) {
  let mut value = bs.get_setting(settings);
  if value < min {
    value = 0;
  }
  if text.is_empty() || &text == "-" {
    value = 0;
  }

  if let Ok(num) = text.trim().parse::<i8>() {
    if num <= max && num >= min {
      value = num;
    }
  }
  bs.set_setting(settings, value);
}

fn float_setting<B: ByteSetting>(
  bs: &mut B,
  settings: &mut HackSettings,
  float: f64
) {
  let value = float.trunc() as i8;
  bs.set_setting(settings, value);
}


impl AllStatTabsSettings {
  fn apply_patch(&mut self) -> std::io::Result<()> {
    let original_rom_str = self.settings.patch.original_rom.clone();
    let modified_rom_str = self.settings.patch.modified_rom.clone();

    let asar_str = if self.settings.patch.asar.is_empty() {
      String::from("asar")
    } else {
      self.settings.patch.asar.clone()
    };

    let patch_str = if self.settings.patch.patch.is_empty() {
      String::from(PATCH_DEFAULT_PATH)
    } else {
      self.settings.patch.patch.clone()
    };

    let original_rom = Path::new(&original_rom_str).canonicalize()?;
    let modified_rom = Path::new(&modified_rom_str).canonicalize()?;
    let asar  = Path::new(&asar_str).canonicalize()?;
    let patch = Path::new(&patch_str).canonicalize()?;

    copy(&original_rom, &modified_rom)?;

    let temp_dir = tempfile::tempdir()?;
    std::env::set_current_dir(&temp_dir)?;
    self.settings.save();

    let cmd = Command::new(&asar)
      .args(&[&patch, &modified_rom])
      .output()?;

    if cmd.status.success() {
      self.patch_status.clear();
      self.patch_status.push_str("Success!\n");
    } else {
      self.patch_status.clear();
      self.patch_status.push_str("Error while patching rom: ");
      let maybe_msg = String::from_utf8(cmd.stderr);
      match maybe_msg {
        Ok(msg) => {
          self.patch_status.push_str(&msg);
        }
        Err(_) => {
          self.patch_status.push_str("??");
        }
      }
    }
    Ok(())
  }

  pub fn react_to_message(&mut self, message: ASTMessage) {
    macro_rules! string_setting {
      ($value: expr, $stat: ident, $min: expr, $max: expr) => {
        string_setting(&mut self.$stat, &mut self.settings, $value, $min, $max);
      }
    }

    macro_rules! float_setting {
      ($value: expr, $stat: ident) => {
        float_setting(&mut self.$stat, &mut self.settings, $value);
      }
    }

    use ASTMessage::*;
    match message {
      SetOriginalRomInput(value) => {
        self.settings.patch.original_rom.clear();
        self.settings.patch.original_rom.push_str(&value);
      }

      SetModifiedRomInput(value) => {
        self.settings.patch.modified_rom.clear();
        self.settings.patch.modified_rom.push_str(&value);
      }

      SetAsar(value) => {
        self.settings.patch.asar.clear();
        self.settings.patch.asar.push_str(&value);
      }

      SetPatch(value) => {
        self.settings.patch.patch.clear();
        self.settings.patch.patch.push_str(&value);
      }

      PressedOriginalRomButton => {
        let result = nfd::open_file_dialog(Some("smc,sfc"), None).unwrap_or_else(|e| {
          eprintln!("Error while opening a file: {}", e);
          nfd::Response::Cancel
        });

        match result {
          nfd::Response::Okay(path) => {
            self.settings.patch.original_rom.clear();
            self.settings.patch.original_rom.push_str(&path);
          }

          _ => {
          }
        }
      }

      PressedModifiedRomButton => {
        let result = nfd::open_save_dialog(Some("smc,sfc"), None).unwrap_or_else(|e| {
          eprintln!("Error while opening a file: {}", e);
          nfd::Response::Cancel
        });

        match result {
          nfd::Response::Okay(path) => {
            self.settings.patch.modified_rom.clear();
            self.settings.patch.modified_rom.push_str(&path);
          }

          _ => {
          }
        }
      }

      PressedAsarButton => {
        let result = nfd::open_file_dialog(None, None).unwrap_or_else(|e| {
          eprintln!("Error while opening a file: {}", e);
          nfd::Response::Cancel
        });

        match result {
          nfd::Response::Okay(path) => {
            self.settings.patch.asar.clear();
            self.settings.patch.asar.push_str(&path);
          }

          _ => {
          }
        }
      }

      PressedPatchButton => {
        let result = nfd::open_file_dialog(Some("asm"), None).unwrap_or_else(|e| {
          eprintln!("Error while opening a file: {}", e);
          nfd::Response::Cancel
        });

        match result {
          nfd::Response::Okay(path) => {
            self.settings.patch.patch.clear();
            self.settings.patch.patch.push_str(&path);
          }

          _ => {
          }
        }
      }

      ApplyPatch => {
        let main_dir = std::env::current_dir().expect("Could not find main directory.");
        if let Err(error) = self.apply_patch() {
          let msg = format!("Error while copying rom: {}", error);
          eprintln!("{}", &msg);
          self.patch_status.clear();
          self.patch_status.push_str(&msg);
        } else {
          self.patch_status.clear();
          self.patch_status.push_str("Success!");
        }
        self.patch_timer = 2;
        std::env::set_current_dir(main_dir).expect("Could not return to main directory.")
      }

      SetAlwaysSave(value) => {
        self.settings.always_save = value;
      },

      SetBetterTabs(value) => {
        self.settings.tabs.better_tabs = value;
      },

      SetPowerTabIncreasesHit(value) => {
        self.settings.tabs.power_tab_increases_hit = value;
      },

      SetJetsOfTimeRando(value) => {
        self.settings.tabs.jets_of_time_rando = value;
      },

      SetAllowStatDecrease(value) => {
        self.settings.tabs.allow_stat_decrease = value;
        if value {
          self.stat_min = -99
        } else {
          self.stat_min = 0
        }
      },

      SetPowerIncrease(value) => {
        string_setting!(value, power_increase, self.stat_min, 99);
      },

      SetPowerIncreaseSlider(value) => {
        float_setting!(value, power_increase);
      },

      SetStaminaIncrease(value) => {
        string_setting!(value, stamina_increase, self.stat_min, 99);
      },

      SetStaminaIncreaseSlider(value) => {
        float_setting!(value, stamina_increase);
      },

      SetSpeedIncrease(value) => {
        string_setting!(value, speed_increase, 16*self.stat_min.signum(), 16);
      },

      SetSpeedIncreaseSlider(value) => {
        float_setting!(value, speed_increase);
      },

      SetMagicIncrease(value) => {
        string_setting!(value, magic_increase, self.stat_min, 99);
      },

      SetMagicIncreaseSlider(value) => {
        float_setting!(value, magic_increase);
      },

      SetHitIncrease(value) => {
        string_setting!(value, hit_increase, self.stat_min, 99);
      },

      SetHitIncreaseSlider(value) => {
        float_setting!(value, hit_increase);
      },

      SetEvadeIncrease(value) => {
        string_setting!(value, evade_increase, self.stat_min, 99);
      },

      SetEvadeIncreaseSlider(value) => {
        float_setting!(value, evade_increase);
      },

      SetMagicDefenseIncrease(value) => {
        string_setting!(value, magic_defense_increase, self.stat_min, 99);
      },

      SetMagicDefenseIncreaseSlider(value) => {
        float_setting!(value, magic_defense_increase);
      },

      SetExpGoldTechAllow(value) => {
        self.settings.expgoldtech.allow = value;
      },

      SetGradualExp(value) => {
        self.settings.expgoldtech.gradual_exp = value;
      },

      SetGradualExpMin(value) => {
        string_setting!(value, gradual_exp_min, 0, 4);
      },

      SetGradualExpMinSlider(value) => {
        float_setting!(value, gradual_exp_min);
      }

      SetExpIncrease(value) => {
        string_setting!(value, exp_increase, 1, 64);
      }

      SetExpIncreaseSlider(value) => {
        float_setting!(value, exp_increase);
      }

      SetGoldIncrease(value) => {
        string_setting!(value, gold_increase, 1, 64);
      }

      SetGoldIncreaseSlider(value) => {
        float_setting!(value, gold_increase);
      }

      SetTechIncrease(value) => {
        string_setting!(value, tech_increase, 1, 64);
      }

      SetTechIncreaseSlider(value) => {
        float_setting!(value, tech_increase);
      }

      Save => {
        self.settings.save()
      }

      Help => {
        self.help = !self.help;
      }

      Default => {
        let always_save = self.settings.always_save;
        let patch = self.settings.patch.clone();
        self.settings = HackSettings::default();
        self.settings.patch = patch;
        self.settings.always_save = always_save;
        self.stat_min = 0;

        self.power_increase.update_input_value(&self.settings);
        self.stamina_increase.update_input_value(&self.settings);
        self.speed_increase.update_input_value(&self.settings);
        self.magic_increase.update_input_value(&self.settings);
        self.hit_increase.update_input_value(&self.settings);
        self.evade_increase.update_input_value(&self.settings);
        self.magic_defense_increase.update_input_value(&self.settings);
        self.gradual_exp_min.update_input_value(&self.settings);
        self.exp_increase.update_input_value(&self.settings);
        self.gold_increase.update_input_value(&self.settings);
        self.tech_increase.update_input_value(&self.settings);
      }

      Quit => {
        exit(0)
      }
    }
    if self.settings.always_save {
      self.settings.save()
    }
  }
}

