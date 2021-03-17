use crate::gui::*;
use crate::gui::view::*;

impl<'a> MainWindowButtons<'a> {
  pub fn into_row(self) -> Row<'a, ASTMessage> {
    Row::new()
    .spacing(10)
    .push(
      Button::new(
        self.help_button_state.unwrap(),
        Text::new("Help").size(TEXT_SIZE)
      )
      .on_press(ASTMessage::Help)
    )
    .push(
      Button::new(
        self.save_button_state.unwrap(),
        Text::new("Save").size(TEXT_SIZE)
      )
      .on_press(ASTMessage::Save)
    )
    .push(
      Button::new(
        self.default_button_state.unwrap(),
        Text::new("Default").size(TEXT_SIZE)
      )
      .on_press(ASTMessage::Default)
    )
    .push(
      Button::new(
        self.quit_button_state.unwrap(),
        Text::new("Quit").size(TEXT_SIZE)
      )
      .on_press(ASTMessage::Quit)
    )
  }
}
