use builder::*;
use crate::gui::*;
use crate::settings::*;

pub trait TextInputMsgFn : 'static + Fn(String) -> ASTMessage {}
impl<T: 'static + Fn(String) -> ASTMessage> TextInputMsgFn for T {}

pub trait SliderMsgFn : 'static + Fn(f64) -> ASTMessage {}
impl<T: 'static + Fn(f64) -> ASTMessage> SliderMsgFn for T {}

pub trait CheckboxMsgFn : 'static + Fn(bool) -> ASTMessage {}
impl<T: 'static + Fn(bool) -> ASTMessage> CheckboxMsgFn for T {}

#[builder]
#[derive(Default)]
pub struct PatchInterface<'a> {
  pub original_rom_state: Option<&'a mut text_input::State>,
  pub original_rom_value: Option<&'a str>,
  pub original_rom_button_state: Option<&'a mut button::State>,

  pub modified_rom_state: Option<&'a mut text_input::State>,
  pub modified_rom_value: Option<&'a str>,
  pub modified_rom_button_state: Option<&'a mut button::State>,

  pub asar_state: Option<&'a mut text_input::State>,
  pub asar_value: Option<&'a str>,
  pub asar_button_state: Option<&'a mut button::State>,

  pub patch_state: Option<&'a mut text_input::State>,
  pub patch_value: Option<&'a str>,
  pub patch_button_state: Option<&'a mut button::State>,

  pub apply_patch_button_state: Option<&'a mut button::State>,
  pub patch_status: Option<&'a str>,
  pub patch_status_scrollable_state: Option<&'a mut scrollable::State>,
}

impl<'a> PatchInterface<'a> {
  pub fn new() -> Self {
    Self::default()
  }
}

#[builder]
pub struct PatchRowInterface<'a, F: TextInputMsgFn> {
  pub text: &'a str,
  pub default: Option<&'a str>,
  pub input_state: Option<&'a mut text_input::State>,
  pub input_value: Option<&'a str>,
  pub input_msg: Option<F>,
  pub button_state: Option<&'a mut button::State>,
  pub button_msg: Option<ASTMessage>,
}

impl<'a, F: TextInputMsgFn> PatchRowInterface<'a, F> {
  pub fn new(text: &'a str) -> Self {
    Self {
      text,
      default: None,
      input_state: None,
      input_value: None,
      input_msg: None,
      button_state: None,
      button_msg: None,
    }
  }
}

#[builder]
#[derive(Default)]
pub struct SettingsInterface<'a> {
  pub settings: HackSettings,
  pub stat_min: Option<i8>,

  pub settings_scrollable_state: Option<&'a mut scrollable::State>,

  pub power_increase_state: Option<&'a mut text_input::State>,
  pub power_increase_value: Option<&'a str>,
  pub power_increase_slider_state: Option<&'a mut slider::State>,

  pub stamina_increase_state: Option<&'a mut text_input::State>,
  pub stamina_increase_value: Option<&'a str>,
  pub stamina_increase_slider_state: Option<&'a mut slider::State>,

  pub speed_increase_state: Option<&'a mut text_input::State>,
  pub speed_increase_value: Option<&'a str>,
  pub speed_increase_slider_state: Option<&'a mut slider::State>,

  pub magic_increase_state: Option<&'a mut text_input::State>,
  pub magic_increase_value: Option<&'a str>,
  pub magic_increase_slider_state: Option<&'a mut slider::State>,

  pub hit_increase_state: Option<&'a mut text_input::State>,
  pub hit_increase_value: Option<&'a str>,
  pub hit_increase_slider_state: Option<&'a mut slider::State>,

  pub evade_increase_state: Option<&'a mut text_input::State>,
  pub evade_increase_value: Option<&'a str>,
  pub evade_increase_slider_state: Option<&'a mut slider::State>,

  pub magic_defense_increase_state: Option<&'a mut text_input::State>,
  pub magic_defense_increase_value: Option<&'a str>,
  pub magic_defense_increase_slider_state: Option<&'a mut slider::State>,

  pub gradual_exp_min_state: Option<&'a mut text_input::State>,
  pub gradual_exp_min_value: Option<&'a str>,
  pub gradual_exp_min_slider_state: Option<&'a mut slider::State>,
}

impl<'a> SettingsInterface<'a> {
  pub fn new(settings: &HackSettings) -> Self {
    Self {
      settings: settings.clone(),
      ..Self::default()
    }
  }
}

#[builder]
pub struct ByteSettingRow<'a, F: TextInputMsgFn, G: SliderMsgFn> {
  pub text: &'a str,
  pub input_state: Option<&'a mut text_input::State>,
  pub input_value: Option<&'a str>,
  pub input_msg: Option<F>,
  pub slider_state: Option<&'a mut slider::State>,
  pub slider_msg: Option<G>,
  pub min: Option<f64>,
  pub max: Option<f64>,
  pub value: Option<f64>,
}

impl<'a, F: TextInputMsgFn, G: SliderMsgFn> ByteSettingRow<'a, F, G> {
  pub fn new(text: &'a str) -> Self {
    Self {
      text,
      input_state: None,
      input_value: None,
      input_msg: None,
      slider_state: None,
      slider_msg: None,
      min: None,
      max: None,
      value: None,
    }
  }
}

#[builder]
pub struct MainWindowButtons<'a> {
  pub settings: HackSettings,
  pub help_button_state: Option<&'a mut button::State>,
  pub save_button_state: Option<&'a mut button::State>,
  pub quit_button_state: Option<&'a mut button::State>,
  pub default_button_state: Option<&'a mut button::State>,
}

impl<'a> MainWindowButtons<'a> {
  pub fn new(settings: &HackSettings) -> Self {
    Self {
      settings: settings.clone(),
      help_button_state: None,
      save_button_state: None,
      quit_button_state: None,
      default_button_state: None,
    }
  }
}

