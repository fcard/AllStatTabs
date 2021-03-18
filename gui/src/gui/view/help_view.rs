use crate::gui::*;
use crate::gui::view::help;

const HELP_CATEGORY_SIZE: u16 = 30;
const HELP_OPTION_SIZE: u16 = 24;
const HELP_TEXT_SIZE: u16 = 16;

fn help_category(name: &str) -> Row<ASTMessage> {
  Row::new()
  .push(Text::new(name).size(HELP_CATEGORY_SIZE))
  .push(Rule::horizontal(HELP_CATEGORY_SIZE))
}

fn help_option_name(name: &str) -> Row<ASTMessage> {
  Row::new()
  .push(Text::new(name).size(HELP_OPTION_SIZE))
  .push(Rule::horizontal(HELP_OPTION_SIZE))
}

fn help_option_text(text: &str) -> Text {
  Text::new(text).size(HELP_TEXT_SIZE)
}

fn help_scrollable(state: &mut scrollable::State) -> Scrollable<ASTMessage> {
  Scrollable::new(state)
  .height(Length::Units(700))
  .push(
    Column::new()
    .padding(20)
    .spacing(10)
    .align_items(Align::Start)
    .push(help_category("Tabs"))
    .push(help_option_name("Better Tabs"))
    .push(help_option_text(help::BETTER_TABS))
    .push(help_option_name("Power Tab Increases Hit"))
    .push(help_option_text(help::POWER_TAB_INCREASES_HIT))
    .push(help_option_name("Jets of Time").width(Length::Units(140)))
    .push(help_option_text(help::JETS_OF_TIME))
    .push(help_option_name("Allow Stat Decrease"))
    .push(help_option_text(help::ALLOW_STAT_DECREASE))
    .push(help_option_name("Stat Increase"))
    .push(help_option_text(help::STAT_INCREASE))
    .push(help_category("Exp/Gold/Tech"))
    .push(help_option_name("Increase Exp/Gold/Tech Points"))
    .push(help_option_text(help::EXPGOLDTECH_ALLOW))
    .push(help_option_name("Gradual Experience Increase"))
    .push(help_option_text(help::GRADUAL_EXP))
    .push(help_option_name("Minimum Gradual Experience Level Group"))
    .push(help_option_text(help::GRADUAL_EXP_MIN))
    .push(help_option_name("Experience Increase"))
    .push(help_option_text(help::EXP_INCREASE))
    .push(help_option_name("Gold Increase"))
    .push(help_option_text(help::GOLD_INCREASE))
    .push(help_option_name("Tech Increase"))
    .push(help_option_text(help::TECH_INCREASE))
  )
}

impl AllStatTabsSettings {
  pub fn help_view(&mut self) -> Element<ASTMessage> {
    Column::new()
      .padding(10)
      .align_items(Align::Center)
      .push(help_scrollable(&mut self.help_scrollable_state))
      .push(
        Container::new(
          Button::new(
            &mut self.help_button_state,
            Text::new("Return")
          )
         .on_press(ASTMessage::Help)
        )
        .padding(20)
      )
      .into()
  }
}
