mod view;
mod update;

use const_format::*;
use pathsepstring::*;

pub use iced::{Sandbox, Element, Column, Row, Scrollable, Container, Align, Length};
pub use iced::{TextInput, Checkbox, Slider, Text, Rule, Button};
pub use iced::{text_input, slider, button, scrollable};

use crate::settings::*;

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
  pub power_increase_value : String,
  pub power_increase_state : text_input::State,
  pub power_increase_slider_state : slider::State,
  pub stamina_increase_value : String,
  pub stamina_increase_state : text_input::State,
  pub stamina_increase_slider_state : slider::State,
  pub speed_increase_value : String,
  pub speed_increase_state : text_input::State,
  pub speed_increase_slider_state : slider::State,
  pub magic_increase_value : String,
  pub magic_increase_state : text_input::State,
  pub magic_increase_slider_state : slider::State,
  pub hit_increase_value : String,
  pub hit_increase_state : text_input::State,
  pub hit_increase_slider_state : slider::State,
  pub evade_increase_value : String,
  pub evade_increase_state : text_input::State,
  pub evade_increase_slider_state : slider::State,
  pub magic_defense_increase_value : String,
  pub magic_defense_increase_state : text_input::State,
  pub magic_defense_increase_slider_state : slider::State,
  pub gradual_exp_min_value : String,
  pub gradual_exp_min_state : text_input::State,
  pub gradual_exp_min_slider_state : slider::State,
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
    let power_increase_value = settings.tabs.power_increase.to_string();
    let stamina_increase_value = settings.tabs.stamina_increase.to_string();
    let speed_increase_value = settings.tabs.speed_increase.to_string();
    let magic_increase_value = settings.tabs.magic_increase.to_string();
    let hit_increase_value = settings.tabs.hit_increase.to_string();
    let evade_increase_value = settings.tabs.evade_increase.to_string();
    let magic_defense_increase_value = settings.tabs.magic_defense_increase.to_string();
    let gradual_exp_min_value = settings.expgoldtech.gradual_exp_min.to_string();
    Self {
      settings,
      stat_min,
      help : false,
      patch_status: String::from("..."),
      patch_timer: 0,
      patch_status_scrollable_state: scrollable::State::new(),
      help_scrollable_state: scrollable::State::new(),
      settings_scrollable_state: scrollable::State::new(),
      original_rom_value: String::new(),
      original_rom_state: text_input::State::new(),
      original_rom_button_state : button::State::new(),
      modified_rom_value: String::new(),
      modified_rom_state: text_input::State::new(),
      modified_rom_button_state: button::State::new(),
      asar_value: String::new(),
      asar_state: text_input::State::new(),
      asar_button_state: button::State::new(),
      patch_value: String::new(),
      patch_state: text_input::State::new(),
      patch_button_state: button::State::new(),
      apply_patch_button_state: button::State::new(),
      power_increase_value,
      power_increase_state: text_input::State::new(),
      power_increase_slider_state: slider::State::new(),
      stamina_increase_value,
      stamina_increase_state: text_input::State::new(),
      stamina_increase_slider_state: slider::State::new(),
      speed_increase_value,
      speed_increase_state: text_input::State::new(),
      speed_increase_slider_state: slider::State::new(),
      magic_increase_value,
      magic_increase_state: text_input::State::new(),
      magic_increase_slider_state: slider::State::new(),
      hit_increase_value,
      hit_increase_state: text_input::State::new(),
      hit_increase_slider_state: slider::State::new(),
      evade_increase_value,
      evade_increase_state: text_input::State::new(),
      evade_increase_slider_state: slider::State::new(),
      magic_defense_increase_value,
      magic_defense_increase_state: text_input::State::new(),
      magic_defense_increase_slider_state: slider::State::new(),
      gradual_exp_min_value,
      gradual_exp_min_state: text_input::State::new(),
      gradual_exp_min_slider_state: slider::State::new(),
      save_button_state: button::State::new(),
      quit_button_state: button::State::new(),
      help_button_state: button::State::new(),
      default_button_state: button::State::new(),
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


