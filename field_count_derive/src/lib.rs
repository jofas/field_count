extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(FieldCount)]
pub fn derive_field_count(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);

  let name = &input.ident;

  let field_count = if let Data::Struct(data_struct) = input.data {
    match data_struct.fields {
      Fields::Named(fields) => fields.named.len(),
      _ => panic!("derive(FieldCount) only possible on named structs")
    }
  } else {
    panic!("derive(FieldCount) only possible on named structs");
  };


  let result = quote! {
    impl FieldCount for #name {
      fn field_count() -> usize {
        #field_count
      }
    }
  };

  TokenStream::from(result)
}
