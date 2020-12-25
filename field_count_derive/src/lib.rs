extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Type,
  PathArguments};

use std::collections::HashMap;

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

#[proc_macro_derive(FieldCountByType)]
pub fn derive_field_count_by_type(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);

  let name = &input.ident;

  // type -> (args_count, count)
  let mut generic_types = HashMap::new();
  let mut types = HashMap::new();

  // IMPORTANT: this macro will fail when same type is used with
  // two different paths (e.g. struct Example(String,
  // ::std::string::String);) will fail
  //
  // TODO: Option<_> to count over all option types
  match input.data {
    Data::Struct(data_struct) => match data_struct.fields {
      Fields::Named(fields) =>
        fields.named.into_iter().for_each(|x| {

          //println!("{:?}", x.ty);

          if let Type::Path(path) = &x.ty {
            let ident = path.path.segments.last().unwrap();
            if let PathArguments::AngleBracketed(_args) =
              &ident.arguments
            {
              let count = *generic_types.get(&ident.ident)
                .unwrap_or(&0_usize);
              generic_types.insert(ident.ident.clone(), count + 1);
            }
          }

          let count = *types.get(&x.ty).unwrap_or(&0_usize);
          types.insert(x.ty, count + 1);

        }),
      Fields::Unnamed(fields) =>
        fields.unnamed.into_iter().for_each(|x| {
          let count = *types.get(&x.ty).unwrap_or(&0_usize);
          types.insert(x.ty, count + 1);
        }),
      Fields::Unit => (),
    },
    _ => panic!("Derive(FieldCountByType) only applicable to structs"),
  }

  let keys = types.keys();
  let values = types.values();

  let generic_keys = generic_types.keys();
  let generic_values = generic_types.values();

  println!("{:?}", generic_types);

  let result = quote! {
    #(
      impl FieldCountByType<#keys> for #name {
        fn field_count_by_type(&self) -> usize {#values}
      }
     )*
    #(
      impl FieldCountByType<#generic_keys<field_count::Generic>> for #name {
        fn field_count_by_type(&self) -> usize {#generic_values}
      }
    )*
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

        quote! {
          impl RecursiveFieldCount for #name {
            fn recursive_field_count(&self) -> usize {
              0 #(
               + self.#fields.recursive_field_count()
              )*
            }
          }
        }
      },
      _ => panic!(ERR_MSG),
    },
    _ => panic!(ERR_MSG),
  };

  TokenStream::from(result)
}
