use crate::gui::*;

use std::process::exit;
use std::process::Command;
use std::fs::copy;
use std::path::Path;

impl AllStatTabsSettings {
  fn string_to_stat(&mut self, text: String, mut default: i8, max: i8) -> i8 {
    if default < self.stat_min {
      default = 0
    }
    if text.is_empty() || &text == "-" {
      return 0;
    }

    if let Ok(num) = text.trim().parse::<i8>() {
      if num > max {
        default
      } else if num < max*self.stat_min.signum() {
        default
      } else {
        num
      }
    } else {
      default
    }
  }

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
    macro_rules! stat_increase {
      ($value: expr, $max: expr, $input_value: ident, $setting: ident) => {
        let n = self.string_to_stat($value, self.settings.tabs.$setting, $max);
        stat_increase_general!(n, $input_value, $setting);
      }
    }

    macro_rules! stat_increase_slider {
      ($value: expr, $input_value: ident, $setting: ident) => {
        let n = $value.trunc() as i8;
        stat_increase_general!(n, $input_value, $setting);
      }
    }

    macro_rules! stat_increase_general {
      ($n: expr, $input_value: ident, $setting: ident) => {
        self.$input_value.clear();
        self.$input_value.push_str(&$n.to_string());
        self.settings.tabs.$setting = $n;
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
        stat_increase!(value, 99, power_increase_value, power_increase);
      },

      SetPowerIncreaseSlider(value) => {
        stat_increase_slider!(value, power_increase_value, power_increase);
      },

      SetStaminaIncrease(value) => {
        stat_increase!(value, 99, stamina_increase_value, stamina_increase);
      },

      SetStaminaIncreaseSlider(value) => {
        stat_increase_slider!(value, stamina_increase_value, stamina_increase);
      },

      SetSpeedIncrease(value) => {
        stat_increase!(value, 16, speed_increase_value, speed_increase);
      },

      SetSpeedIncreaseSlider(value) => {
        stat_increase_slider!(value, speed_increase_value, speed_increase);
      },

      SetMagicIncrease(value) => {
        stat_increase!(value, 99, magic_increase_value, magic_increase);
      },

      SetMagicIncreaseSlider(value) => {
        stat_increase_slider!(value, magic_increase_value, magic_increase);
      },

      SetHitIncrease(value) => {
        stat_increase!(value, 99, hit_increase_value, hit_increase);
      },

      SetHitIncreaseSlider(value) => {
        stat_increase_slider!(value, hit_increase_value, hit_increase);
      },

      SetEvadeIncrease(value) => {
        stat_increase!(value, 99, evade_increase_value, evade_increase);
      },

      SetEvadeIncreaseSlider(value) => {
        stat_increase_slider!(value, evade_increase_value, evade_increase);
      },

      SetMagicDefenseIncrease(value) => {
        stat_increase!(value, 99, magic_defense_increase_value, magic_defense_increase);
      },

      SetMagicDefenseIncreaseSlider(value) => {
        stat_increase_slider!(value, magic_defense_increase_value, magic_defense_increase);
      },

      SetExpGoldTechAllow(value) => {
        self.settings.expgoldtech.allow = value;
      },

      SetGradualExp(value) => {
        self.settings.expgoldtech.gradual_exp = value;
      },

      SetGradualExpMin(mut value) => {
        if value.is_empty() {
          value = String::from("0");
        }
        if let Ok(n) = value.parse::<i8>() {
          if n >= 0 && n <= 4 {
            self.settings.expgoldtech.gradual_exp_min = n;
            self.gradual_exp_min_value.clear();
            self.gradual_exp_min_value.push_str(&n.to_string());
          }
        }
      },

      SetGradualExpMinSlider(value) => {
        let n = value.trunc() as i8;
        self.settings.expgoldtech.gradual_exp_min = n;
        self.gradual_exp_min_value.clear();
        self.gradual_exp_min_value.push_str(&n.to_string());
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

        let power           = self.settings.tabs.power_increase.to_string();
        let stamina         = self.settings.tabs.stamina_increase.to_string();
        let speed           = self.settings.tabs.speed_increase.to_string();
        let magic           = self.settings.tabs.magic_increase.to_string();
        let hit             = self.settings.tabs.hit_increase.to_string();
        let evade           = self.settings.tabs.evade_increase.to_string();
        let magic_defense   = self.settings.tabs.magic_defense_increase.to_string();
        let gradual_exp_min = self.settings.expgoldtech.gradual_exp_min.to_string();

        self.power_increase_value.clear();
        self.power_increase_value.push_str(&power);
        self.stamina_increase_value.clear();
        self.stamina_increase_value.push_str(&stamina);
        self.speed_increase_value.clear();
        self.speed_increase_value.push_str(&speed);
        self.magic_increase_value.clear();
        self.magic_increase_value.push_str(&magic);
        self.hit_increase_value.clear();
        self.hit_increase_value.push_str(&hit);
        self.evade_increase_value.clear();
        self.evade_increase_value.push_str(&evade);
        self.magic_defense_increase_value.clear();
        self.magic_defense_increase_value.push_str(&magic_defense);
        self.gradual_exp_min_value.clear();
        self.gradual_exp_min_value.push_str(&gradual_exp_min);
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

