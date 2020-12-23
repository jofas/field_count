extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(FieldCount)]
pub fn derive_field_count(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);

  let name = &input.ident;

  let field_count = match input.data {
    Data::Struct(data_struct) => match data_struct.fields {
      Fields::Named(fields) => fields.named.len(),
      Fields::Unnamed(fields) => fields.unnamed.len(),
      Fields::Unit => 0,
    },
    _ => panic!("Derive(FieldCount) only applicable to structs"),
  };

  let result = quote! {
    impl FieldCount for #name {
      fn field_count(&self) -> usize {
        #field_count
      }
    }
  };

  TokenStream::from(result)
}

static ERR_MSG: &'static str =
  "Derive(RecursiveFieldCount) only applicable to named structs";

#[proc_macro_derive(RecursiveFieldCount)]
pub fn derive_recursive_field_count(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);

  let name = &input.ident;

  let result = match input.data {
    Data::Struct(data_struct) => match data_struct.fields {
      Fields::Named(fields) => {
        let fields: Vec<syn::Ident> = fields.named.iter().map(
          |field| field.ident.clone().unwrap()
        ).collect();
        println!("{:?}", fields);
        let res = quote! {
          impl RecursiveFieldCount for #name {
            fn recursive_field_count(&self) -> usize {
              0 #(
               + self.#fields.field_count()
              )*
            }
          }
        };
        println!("{}", res);
        res
      },
      _ => panic!(ERR_MSG),
    },
    _ => panic!(ERR_MSG),
  };

  TokenStream::from(result)
}
