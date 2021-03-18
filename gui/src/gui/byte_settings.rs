use crate::gui::*;

pub trait ByteSetting {
  fn new(settings: &HackSettings) -> Self;
  fn parts(&mut self) -> ByteSettingParts;
  fn get_setting(&self, settings: &HackSettings) -> i8;
  fn set_setting(&mut self, settings: &mut HackSettings, value: i8);
  fn update_input_value(&mut self, settings: &HackSettings);
}

pub struct ByteSettingParts<'a> {
  pub input_value: &'a str,
  pub input_state: &'a mut text_input::State,
  pub slider_state: &'a mut slider::State,
}

macro_rules! create_byte_setting_struct {
  ($name: ident, $($fields: tt)*) => {
    #[derive(Default)]
    pub struct $name {
      input_value: String,
      input_state: text_input::State,
      slider_state: slider::State,
    }

    impl ByteSetting for $name {
      fn new(settings: &HackSettings) -> Self {
        Self {
          input_value: settings$($fields)*.to_string(),
          ..Self::default()
        }
      }

      fn parts(&mut self) -> ByteSettingParts {
        ByteSettingParts {
          input_value: &self.input_value,
          input_state: &mut self.input_state,
          slider_state: &mut self.slider_state,
        }
      }


      fn get_setting(&self, settings: &HackSettings) -> i8 {
        settings$($fields)*
      }

      fn set_setting(&mut self, settings: &mut HackSettings, value: i8) {
        if settings$($fields)* != value {
          settings$($fields)* = value;
          self.input_value = value.to_string();
        }
      }

      fn update_input_value(&mut self, settings: &HackSettings) {
        let value = self.get_setting(settings);
        self.input_value = value.to_string();
      }
    }
  }
}

create_byte_setting_struct!(PowerIncrease,        .tabs.power_increase);
create_byte_setting_struct!(StaminaIncrease,      .tabs.stamina_increase);
create_byte_setting_struct!(MagicIncrease,        .tabs.magic_increase);
create_byte_setting_struct!(SpeedIncrease,        .tabs.speed_increase);
create_byte_setting_struct!(HitIncrease,          .tabs.hit_increase);
create_byte_setting_struct!(EvadeIncrease,        .tabs.evade_increase);
create_byte_setting_struct!(MagicDefenseIncrease, .tabs.magic_defense_increase);
create_byte_setting_struct!(GradualExpMin,        .expgoldtech.gradual_exp_min);
create_byte_setting_struct!(ExpIncrease,          .expgoldtech.exp_increase);
create_byte_setting_struct!(GoldIncrease,         .expgoldtech.gold_increase);
create_byte_setting_struct!(TechIncrease,         .expgoldtech.tech_increase);

