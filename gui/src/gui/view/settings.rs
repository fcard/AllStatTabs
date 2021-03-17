use crate::gui::*;
use crate::gui::view::*;

impl<'a, F: TextInputMsgFn, G: SliderMsgFn> ByteSettingRow<'a, F, G> {
  pub fn into_row(self) -> Row<'a, ASTMessage> {
    Row::new()
    .spacing(20)
    .push(
      Text::new(self.text)
      .size(TEXT_SIZE)
      .width(Length::Units(STAT_LABEL_SIZE))
    )
    .push(
      TextInput::new(
        self.input_state.unwrap(),
        "",
        self.input_value.unwrap(),
        self.input_msg.unwrap()
      )
      .size(TEXT_SIZE)
      .padding(2)
      .width(Length::Units(STAT_INPUT_SIZE))
    )
    .push(
      Slider::new(
        self.slider_state.unwrap(),
        (self.min.unwrap())..=(self.max.unwrap()),
        self.value.unwrap(),
        self.slider_msg.unwrap()
      )
    )
  }
}

pub fn settings_header<'a>() -> Row<'a, ASTMessage> {
  Row::new()
  .push(Rule::horizontal(40))
  .push(Text::new("Settings").size(40))
  .push(Rule::horizontal(40))
}

pub fn settings_category(name: &str) -> Row<ASTMessage> {
  Row::new()
  .padding(10)
  .push(Text::new(name).size(32))
  .push(Rule::horizontal(32))
}

pub fn always_save_checkbox<'a>(always_save: bool) -> Row<'a, ASTMessage> {
  Row::new()
  .padding(10)
  .push(Text::new(String::new()).width(Length::Fill))
  .push(
    Checkbox::new(
      always_save,
      "Save After Every Change",
      ASTMessage::SetAlwaysSave
    )
    .size(TEXT_SIZE)
    .text_size(TEXT_SIZE)
  )
}

pub fn tab_checkbox_settings<'a>(settings: HackSettings) -> Column<'a, ASTMessage> {
  Column::new()
  .spacing(10)
  .padding(20)
  .align_items(Align::Center)
  .push(
    Row::new()
    .spacing(30)
    .push(
      settings_checkbox(
        "Better Tabs",
        settings.tabs.better_tabs,
        ASTMessage::SetBetterTabs
      )
    )
    .push(
      settings_checkbox(
        "Power Tab Increases Hit",
        settings.tabs.power_tab_increases_hit,
        ASTMessage::SetPowerTabIncreasesHit
      )
    )
    .push(
      settings_checkbox(
        "Jets of Time",
        settings.tabs.jets_of_time_rando,
        ASTMessage::SetJetsOfTimeRando
      )
      .width(Length::Units(150))
    )
  )
  .push(
    Row::new()
    .push(
      settings_checkbox(
        "Allow Stat Decrease",
        settings.tabs.allow_stat_decrease,
        ASTMessage::SetAllowStatDecrease
      )
    )
    .push(
      Text::new("").width(Length::Units(40))
    )
  )
}

pub fn settings_checkbox<F: CheckboxMsgFn>(text: &str, value: bool, msg: F) -> Checkbox<ASTMessage> {
  Checkbox::new(value, text, msg)
  .size(TEXT_SIZE)
  .text_size(TEXT_SIZE)
  .spacing(8)
}

impl<'a> SettingsInterface<'a> {
  pub fn into_column(self) -> Column<'a, ASTMessage> {
    Column::new()
    .padding(20)
    .spacing(10)
    .align_items(Align::Start)
    .height(Length::Units(460))
    .push(settings_header())
    .push(
      Scrollable::new(self.settings_scrollable_state.unwrap())
      .spacing(10)
      .padding(20)
      .align_items(Align::Start)
      .push(always_save_checkbox(self.settings.always_save))
      .push(settings_category("Tabs"))
      .push(tab_checkbox_settings(self.settings.clone()))
      .push(
        ByteSettingRow::new("Power Increase")
        .input_state(self.power_increase_state.unwrap())
        .input_value(self.power_increase_value.unwrap())
        .input_msg(ASTMessage::SetPowerIncrease)
        .slider_state(self.power_increase_slider_state.unwrap())
        .slider_msg(ASTMessage::SetPowerIncreaseSlider)
        .min(self.stat_min.unwrap() as f64)
        .max(99.0)
        .value(self.settings.tabs.power_increase as f64)
        .into_row()
      )
      .push(
        ByteSettingRow::new("Stamina Increase")
        .input_state(self.stamina_increase_state.unwrap())
        .input_value(self.stamina_increase_value.unwrap())
        .input_msg(ASTMessage::SetStaminaIncrease)
        .slider_state(self.stamina_increase_slider_state.unwrap())
        .slider_msg(ASTMessage::SetStaminaIncreaseSlider)
        .min(self.stat_min.unwrap() as f64)
        .max(99.0)
        .value(self.settings.tabs.stamina_increase as f64)
        .into_row()
      )
      .push(
        ByteSettingRow::new("Speed Increase")
        .input_state(self.speed_increase_state.unwrap())
        .input_value(self.speed_increase_value.unwrap())
        .input_msg(ASTMessage::SetSpeedIncrease)
        .slider_state(self.speed_increase_slider_state.unwrap())
        .slider_msg(ASTMessage::SetSpeedIncreaseSlider)
        .min((16*self.stat_min.unwrap().signum()) as f64)
        .max(16.0)
        .value(self.settings.tabs.speed_increase as f64)
        .into_row()
      )
      .push(
        ByteSettingRow::new("Magic Increase")
        .input_state(self.magic_increase_state.unwrap())
        .input_value(self.magic_increase_value.unwrap())
        .input_msg(ASTMessage::SetMagicIncrease)
        .slider_state(self.magic_increase_slider_state.unwrap())
        .slider_msg(ASTMessage::SetMagicIncreaseSlider)
        .min(self.stat_min.unwrap() as f64)
        .max(99.0)
        .value(self.settings.tabs.magic_increase as f64)
        .into_row()
      )
      .push(
        ByteSettingRow::new("Hit Increase")
        .input_state(self.hit_increase_state.unwrap())
        .input_value(self.hit_increase_value.unwrap())
        .input_msg(ASTMessage::SetHitIncrease)
        .slider_state(self.hit_increase_slider_state.unwrap())
        .slider_msg(ASTMessage::SetHitIncreaseSlider)
        .min(self.stat_min.unwrap() as f64)
        .max(99.0)
        .value(self.settings.tabs.hit_increase as f64)
        .into_row()
      )
      .push(
        ByteSettingRow::new("Evade Increase")
        .input_state(self.evade_increase_state.unwrap())
        .input_value(self.evade_increase_value.unwrap())
        .input_msg(ASTMessage::SetEvadeIncrease)
        .slider_state(self.evade_increase_slider_state.unwrap())
        .slider_msg(ASTMessage::SetEvadeIncreaseSlider)
        .min(self.stat_min.unwrap() as f64)
        .max(99.0)
        .value(self.settings.tabs.evade_increase as f64)
        .into_row()
      )
      .push(
        ByteSettingRow::new("Magic Defense Increase")
        .input_state(self.magic_defense_increase_state.unwrap())
        .input_value(self.magic_defense_increase_value.unwrap())
        .input_msg(ASTMessage::SetMagicDefenseIncrease)
        .slider_state(self.magic_defense_increase_slider_state.unwrap())
        .slider_msg(ASTMessage::SetMagicDefenseIncreaseSlider)
        .min(self.stat_min.unwrap() as f64)
        .max(99.0)
        .value(self.settings.tabs.magic_defense_increase as f64)
        .into_row()
      )
      .push(settings_category("Exp/Gold/Tech"))
      .push(
        Row::new()
        .spacing(30)
        .push(
          settings_checkbox(
            "Increase Exp/Gold/Tech points",
            self.settings.expgoldtech.allow,
            ASTMessage::SetExpGoldTechAllow
          )
        )
        .push(
          settings_checkbox(
            "Gradual Experience Increase",
            self.settings.expgoldtech.gradual_exp,
            ASTMessage::SetGradualExp
          )
        )
      )
      .push(Row::new().padding(4))
      .push(
        ByteSettingRow::new("Minimum Gradual Experience Level Group")
        .input_state(self.gradual_exp_min_state.unwrap())
        .input_value(self.gradual_exp_min_value.unwrap())
        .input_msg(ASTMessage::SetGradualExpMin)
        .slider_state(self.gradual_exp_min_slider_state.unwrap())
        .slider_msg(ASTMessage::SetGradualExpMinSlider)
        .min(0.0)
        .max(4.0)
        .value(self.settings.expgoldtech.gradual_exp_min as f64)
        .into_row()
      )
    )
  }
}
