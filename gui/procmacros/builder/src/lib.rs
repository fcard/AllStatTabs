extern crate proc_macro;
use proc_macro::TokenStream;
use syn::*;
use quote::*;

#[proc_macro_attribute]
pub fn builder(_attr: TokenStream, input: TokenStream) -> TokenStream {
  match parse_macro_input!(input as Item) {
    Item::Struct(struct_def) => {
      let ident = &struct_def.ident;
      let generics = &struct_def.generics;
      let generics_idents = generics_idents(generics);
      let methods = builder_methods(&struct_def);

      (quote! {
        #struct_def
        impl#generics #ident#generics_idents {
          #methods
        }
      }).into()
    }
    _ => {
      panic!("item type not supported for builder attribute")
    }
  }
}

fn generics_idents(generics: &Generics) -> proc_macro2::TokenStream {
  let lifetimes: Vec<_> = generics.lifetimes().collect();
  let params = generics.type_params().map(|type_param| {
    &type_param.ident
  });
  if lifetimes.is_empty() {
    quote! {
      <#(#params),*>
    }
  } else {
    quote! {
      <#(#lifetimes),*,#(#params),*>
    }
  }
}

fn is_option(ty: &Type) -> bool {
  match ty {
    Type::Paren(paren) => {
      is_option(&*paren.elem)
    }

    Type::Group(group) => {
      is_option(&*group.elem)
    }

    Type::Path(path) => {
      if let Some(segment) = path.path.segments.first() {
        &segment.ident.to_string() == "Option"
      } else {
        false
      }
    }

    _ => {
      false
    }
  }
}

fn unwrap_option_type(ty: &Type) -> Type {
  match ty {
    Type::Paren(paren) => {
      unwrap_option_type(&*paren.elem)
    }

    Type::Group(group) => {
      unwrap_option_type(&*group.elem)
    }

    Type::Path(path) => {
      if let Some(segment) = path.path.segments.first() {
        if let PathArguments::AngleBracketed(args) = &segment.arguments {
          if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
            return inner_ty.clone();
          }
        }
      }
      panic!("Invalid type");
    }

    _ => {
      panic!("Invalid type");
    }
  }
}

fn builder_methods(struct_def: &ItemStruct) -> proc_macro2::TokenStream {
  let methods = struct_def.fields.iter().filter_map(|field| {
    if is_option(&field.ty) {
      if let Some(ident) = &field.ident {
        let ty = unwrap_option_type(&field.ty);
        Some(builder_method(&ident, &ty))
      } else {
        None
      }
    } else {
      None
    }
  });
  quote! {
    #(#methods)*
  }
}

fn builder_method(field: &Ident, ty: &Type) -> proc_macro2::TokenStream {

  quote! {
    pub fn #field(mut self, value: #ty) -> Self {
      self.#field = Some(value);
      self
    }
  }
}
