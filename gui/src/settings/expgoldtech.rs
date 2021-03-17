use crate::settings::file::*;

#[derive(Clone)]
pub struct ExpGoldTechSettings {
  pub allow : bool,
  pub gradual_exp : bool,
  pub gradual_exp_min : i8
}

impl ExpGoldTechSettings {
  pub fn new(settings: &KeyValuePairs) -> Self {
    Self {
      allow:           settings.get_bool("IncreaseExpGoldTech", false),
      gradual_exp:     settings.get_bool("GradualExpIncrease", false),
      gradual_exp_min: settings.get_signed_byte("GradualExpMin", 0),
    }
  }
}

