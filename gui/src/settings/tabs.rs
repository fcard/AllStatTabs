use crate::settings::file::*;

#[derive(Clone)]
pub struct TabsSettings {
  pub better_tabs : bool,
  pub power_tab_increases_hit : bool,
  pub jets_of_time_rando : bool,
  pub allow_stat_decrease : bool,
  pub power_increase : i8,
  pub stamina_increase : i8,
  pub speed_increase : i8,
  pub magic_increase : i8,
  pub hit_increase : i8,
  pub evade_increase : i8,
  pub magic_defense_increase : i8,
}

impl TabsSettings {
  pub fn new(settings: &KeyValuePairs) -> Self {
    Self {
      better_tabs:             settings.get_bool("BetterTabs", true),
      power_tab_increases_hit: settings.get_bool("PowerTabIncreasesHit", false),
      jets_of_time_rando:      settings.get_bool("JetsOfTime", false),
      allow_stat_decrease:     settings.get_bool("AllowStatDecrease", false),
      power_increase:          settings.get_signed_byte("PowerIncrease", 2),
      stamina_increase:        settings.get_signed_byte("StaminaIncrease", 2),
      speed_increase:          settings.get_signed_byte("SpeedIncrease", 1),
      magic_increase:          settings.get_signed_byte("MagicIncrease", 2),
      hit_increase:            settings.get_signed_byte("HitIncrease", 2),
      evade_increase:          settings.get_signed_byte("EvadeIncrease", 2),
      magic_defense_increase:  settings.get_signed_byte("MagicDefenseIncrease", 2),
    }
  }
}
