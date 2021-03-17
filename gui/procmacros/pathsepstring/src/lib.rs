use proc_macro::*;

#[proc_macro]
pub fn pathsepstring(input: TokenStream) -> TokenStream {
  if !input.is_empty() {
    panic!("macro pathsepstring! doesn't take any arguments")
  }

  let sep = std::path::MAIN_SEPARATOR.to_string();
  (quote::quote! {
    #sep
  }).into()
}
