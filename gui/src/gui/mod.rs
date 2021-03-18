mod view;
mod update;
mod byte_settings;

pub use byte_settings::*;

use const_format::*;
use pathsepstring::*;

pub use iced::{Sandbox, Element, Column, Row, Scrollable, Container, Align, Length};
pub use iced::{TextInput, Checkbox, Slider, Text, Rule, Button};
pub use iced::{text_input, slider, button, scrollable};

use crate::settings::*;

#[derive(Default)]
pub struct AllStatTabsSettings {
  pub settings : HackSettings,
  pub stat_min : i8,
  pub help : bool,
  pub patch_status : String,
  pub patch_timer : u64,
  pub patch_status_scrollable_state : scrollable::State,
  pub help_scrollable_state : scrollable::State,
  pub settings_scrollable_state : scrollable::State,
  pub original_rom_value : String,
  pub original_rom_state : text_input::State,
  pub original_rom_button_state : button::State,
  pub modified_rom_value : String,
  pub modified_rom_state : text_input::State,
  pub modified_rom_button_state : button::State,
  pub apply_patch_button_state : button::State,
  pub asar_value : String,
  pub asar_state : text_input::State,
  pub asar_button_state : button::State,
  pub patch_value : String,
  pub patch_state : text_input::State,
  pub patch_button_state : button::State,
  pub power_increase : PowerIncrease,
  pub stamina_increase : StaminaIncrease,
  pub speed_increase : SpeedIncrease,
  pub magic_increase : MagicIncrease,
  pub hit_increase : HitIncrease,
  pub evade_increase : EvadeIncrease,
  pub magic_defense_increase : MagicDefenseIncrease,
  pub gradual_exp_min : GradualExpMin,
  pub exp_increase : ExpIncrease,
  pub gold_increase : GoldIncrease,
  pub tech_increase : TechIncrease,
  pub save_button_state : button::State,
  pub quit_button_state : button::State,
  pub help_button_state : button::State,
  pub default_button_state : button::State,
}


#[derive(Debug, Clone)]
pub enum ASTMessage {
  SetOriginalRomInput(String),
  SetModifiedRomInput(String),
  SetAsar(String),
  SetPatch(String),
  PressedOriginalRomButton,
  PressedModifiedRomButton,
  PressedAsarButton,
  PressedPatchButton,
  ApplyPatch,
  SetAlwaysSave(bool),
  SetBetterTabs(bool),
  SetPowerTabIncreasesHit(bool),
  SetAllowStatDecrease(bool),
  SetJetsOfTimeRando(bool),
  SetPowerIncrease(String),
  SetPowerIncreaseSlider(f64),
  SetStaminaIncrease(String),
  SetStaminaIncreaseSlider(f64),
  SetSpeedIncrease(String),
  SetSpeedIncreaseSlider(f64),
  SetMagicIncrease(String),
  SetMagicIncreaseSlider(f64),
  SetHitIncrease(String),
  SetHitIncreaseSlider(f64),
  SetEvadeIncrease(String),
  SetEvadeIncreaseSlider(f64),
  SetMagicDefenseIncrease(String),
  SetMagicDefenseIncreaseSlider(f64),
  SetExpGoldTechAllow(bool),
  SetGradualExp(bool),
  SetGradualExpMin(String),
  SetGradualExpMinSlider(f64),
  SetExpIncrease(String),
  SetExpIncreaseSlider(f64),
  SetGoldIncrease(String),
  SetGoldIncreaseSlider(f64),
  SetTechIncrease(String),
  SetTechIncreaseSlider(f64),
  Save,
  Help,
  Quit,
  Default,
}

const PATCH_DEFAULT_PATH: &str = formatcp!("src{}patch.asm", pathsepstring!());

const TEXT_SIZE: u16 = 18;
const STAT_LABEL_SIZE: u16 = 200;
const STAT_INPUT_SIZE: u16 = 40;

impl Sandbox for AllStatTabsSettings {
  type Message = ASTMessage;

  fn new() -> Self {
    let settings = HackSettings::new();
    let stat_min = if settings.tabs.allow_stat_decrease {-99} else {0};
    let power_increase = PowerIncrease::new(&settings);
    let stamina_increase = StaminaIncrease::new(&settings);
    let speed_increase = SpeedIncrease::new(&settings);
    let magic_increase = MagicIncrease::new(&settings);
    let hit_increase = HitIncrease::new(&settings);
    let evade_increase = EvadeIncrease::new(&settings);
    let magic_defense_increase = MagicDefenseIncrease::new(&settings);
    let gradual_exp_min = GradualExpMin::new(&settings);
    let exp_increase = ExpIncrease::new(&settings);
    let gold_increase = GoldIncrease::new(&settings);
    let tech_increase = TechIncrease::new(&settings);
    Self {
      settings,
      stat_min,
      help : false,
      patch_status: String::from("..."),
      patch_timer: 0,
      power_increase,
      stamina_increase,
      speed_increase,
      magic_increase,
      hit_increase,
      evade_increase,
      magic_defense_increase,
      gradual_exp_min,
      exp_increase,
      gold_increase,
      tech_increase,
      ..Self::default()
    }
  }

  fn title(&self) -> String {
    String::from("All Stat Tabs Settings")
  }

  fn update(&mut self, message: Self::Message) {
    self.react_to_message(message);
  }

  fn view(&mut self) -> Element<Self::Message> {
    if self.help {
      return self.help_view();
    } else {
      return self.main_view();
    }
  }
}


