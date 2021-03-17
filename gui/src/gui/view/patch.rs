use crate::gui::*;
use crate::gui::view::*;

impl<'a, F: TextInputMsgFn> PatchRowInterface<'a, F> {
  pub fn into_row(self) -> Row<'a, ASTMessage> {
    Row::new()
    .spacing(10)
    .push(
      Text::new(self.text).size(TEXT_SIZE)
      .width(Length::Units(110))
    )
    .push(
      TextInput::new(
        self.input_state.unwrap(),
        self.default.unwrap_or(""),
        self.input_value.unwrap_or(""),
        self.input_msg.unwrap()
      )
      .padding(2)
      .size(TEXT_SIZE)
    )
    .push(
      Button::new(
        self.button_state.unwrap(),
        Text::new("...").size(TEXT_SIZE)
      )
      .on_press(self.button_msg.unwrap())
    )
  }
}

impl<'a> PatchInterface<'a> {
  pub fn into_column(self) -> Column<'a, ASTMessage> {
    Column::new()
    .padding(20)
    .spacing(10)
    .align_items(Align::Center)
    .push(
      PatchRowInterface::new("Original Rom")
      .input_state(self.original_rom_state.unwrap())
      .input_value(self.original_rom_value.unwrap())
      .input_msg(ASTMessage::SetOriginalRomInput)
      .button_state(self.original_rom_button_state.unwrap())
      .button_msg(ASTMessage::PressedOriginalRomButton)
      .into_row()
    )
    .push(
      PatchRowInterface::new("Modified Rom")
      .input_state(self.modified_rom_state.unwrap())
      .input_value(self.modified_rom_value.unwrap())
      .input_msg(ASTMessage::SetModifiedRomInput)
      .button_state(self.modified_rom_button_state.unwrap())
      .button_msg(ASTMessage::PressedModifiedRomButton)
      .into_row()
    )
    .push(
      PatchRowInterface::new("Asar Binary")
      .default("asar")
      .input_state(self.asar_state.unwrap())
      .input_value(self.asar_value.unwrap())
      .input_msg(ASTMessage::SetAsar)
      .button_state(self.asar_button_state.unwrap())
      .button_msg(ASTMessage::PressedAsarButton)
      .into_row()
    )
    .push(
      PatchRowInterface::new("patch.asm")
      .default(PATCH_DEFAULT_PATH)
      .input_state(self.patch_state.unwrap())
      .input_value(self.patch_value.unwrap())
      .input_msg(ASTMessage::SetPatch)
      .button_state(self.patch_button_state.unwrap())
      .button_msg(ASTMessage::PressedPatchButton)
      .into_row()
    )
    .push(
      Button::new(
        self.apply_patch_button_state.unwrap(),
        Text::new("Apply Patch").size(TEXT_SIZE)
      )
      .on_press(ASTMessage::ApplyPatch)
    )
    .push(
      Scrollable::new(self.patch_status_scrollable_state.unwrap())
      .height(Length::Units(34))
      .push(Text::new(self.patch_status.unwrap()).size(TEXT_SIZE))
    )
  }
}

