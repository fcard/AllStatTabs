mod types;
mod help_view;
mod help;
mod patch;
mod settings;
mod buttons;

use crate::gui::*;
use types::*;

impl AllStatTabsSettings {
  pub fn main_view(&mut self) -> Element<ASTMessage> {
    if self.patch_timer > 0 {
      self.patch_timer -= 1;
      if self.patch_timer == 0 {
        self.patch_status = String::from("...");
      }
    }

    Column::new()
    .align_items(Align::Center)
    .push(
      PatchInterface::new()
      .original_rom_state(&mut self.original_rom_state)
      .original_rom_value(&self.settings.patch.original_rom)
      .original_rom_button_state(&mut self.original_rom_button_state)
      .modified_rom_state(&mut self.modified_rom_state)
      .modified_rom_value(&self.settings.patch.modified_rom)
      .modified_rom_button_state(&mut self.modified_rom_button_state)
      .asar_state(&mut self.asar_state)
      .asar_value(&self.settings.patch.asar)
      .asar_button_state(&mut self.asar_button_state)
      .patch_state(&mut self.patch_state)
      .patch_value(&self.settings.patch.patch)
      .patch_button_state(&mut self.patch_button_state)
      .apply_patch_button_state(&mut self.apply_patch_button_state)
      .patch_status(&self.patch_status)
      .patch_status_scrollable_state(&mut self.patch_status_scrollable_state)
      .into_column()
    )
    .push(
      SettingsInterface::new(&self.settings)
      .stat_min(self.stat_min)
      .settings_scrollable_state(&mut self.settings_scrollable_state)
      .power_increase_state(&mut self.power_increase_state)
      .power_increase_value(&mut self.power_increase_value)
      .power_increase_slider_state(&mut self.power_increase_slider_state)
      .stamina_increase_state(&mut self.stamina_increase_state)
      .stamina_increase_value(&mut self.stamina_increase_value)
      .stamina_increase_slider_state(&mut self.stamina_increase_slider_state)
      .speed_increase_state(&mut self.speed_increase_state)
      .speed_increase_value(&mut self.speed_increase_value)
      .speed_increase_slider_state(&mut self.speed_increase_slider_state)
      .magic_increase_state(&mut self.magic_increase_state)
      .magic_increase_value(&mut self.magic_increase_value)
      .magic_increase_slider_state(&mut self.magic_increase_slider_state)
      .hit_increase_state(&mut self.hit_increase_state)
      .hit_increase_value(&mut self.hit_increase_value)
      .hit_increase_slider_state(&mut self.hit_increase_slider_state)
      .evade_increase_state(&mut self.evade_increase_state)
      .evade_increase_value(&mut self.evade_increase_value)
      .evade_increase_slider_state(&mut self.evade_increase_slider_state)
      .magic_defense_increase_state(&mut self.magic_defense_increase_state)
      .magic_defense_increase_value(&mut self.magic_defense_increase_value)
      .magic_defense_increase_slider_state(&mut self.magic_defense_increase_slider_state)
      .gradual_exp_min_state(&mut self.gradual_exp_min_state)
      .gradual_exp_min_value(&mut self.gradual_exp_min_value)
      .gradual_exp_min_slider_state(&mut self.gradual_exp_min_slider_state)
      .into_column()
    )
    .push(
      MainWindowButtons::new(&self.settings)
      .help_button_state(&mut self.help_button_state)
      .save_button_state(&mut self.save_button_state)
      .quit_button_state(&mut self.quit_button_state)
      .default_button_state(&mut self.default_button_state)
      .into_row()
    )
    .into()
  }
}
