extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use quote::ToTokens;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Type,
  PathArguments, GenericArgument, parse_quote};
use syn::punctuated::Punctuated;
use syn::token::Comma;

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
    impl field_count::FieldCount for #name {
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

        quote! {
          impl field_count::RecursiveFieldCount for #name {
            fn recursive_field_count(&self) -> usize {
              0 #(
               + self.#fields.recursive_field_count()
              )*
            }
          }
        }
      },
      // TODO: implement for other struct types
      _ => panic!(ERR_MSG),
    },
    _ => panic!(ERR_MSG),
  };

  TokenStream::from(result)
}

fn generate_generic_idents(args: &Punctuated<GenericArgument, Comma>)
  -> Vec<Punctuated<GenericArgument, Comma>>
{
  let mut res = vec![args.clone()];

  for i in 0..args.len() {
    let mut new_args = Vec::new();
    for ident in &res {
      let mut new_ident = ident.clone();
      new_ident[i] = parse_quote!(field_count::Generic);
      new_args.push(new_ident);
    }
    res.append(&mut new_args);
  }

  res
}

#[proc_macro_derive(FieldCountByType)]
pub fn derive_field_count_by_type(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);

  let name = &input.ident;

  let mut types = HashMap::new();

  // IMPORTANT: this macro will fail when same type is used with
  // two different paths (e.g. struct Example(String,
  // ::std::string::String);) will fail
  //
  // TODO: arbitrary Generics depth-wise (Option<Option<Generics>>)
  match input.data {
    Data::Struct(data_struct) => match data_struct.fields {
      Fields::Named(fields) =>
        fields.named.into_iter().for_each(|x| {
          if let Type::Path(type_path) = &x.ty {
            let mut base_ident = match type_path.path.leading_colon {
              Some(_) => String::from("::"),
              None => String::new(),
            };

            let mut new_idents = Vec::new();

            type_path.path.segments.pairs().for_each(|pair| {
              base_ident = if let Some(_) = pair.punct() {
                format!("{}{}::", base_ident, pair.value().ident)
              } else {
                format!("{}{}", base_ident, pair.value().ident)
              };


              if let PathArguments::AngleBracketed(args) =
                &pair.value().arguments
              {
                new_idents =
                  generate_generic_idents(&args.args).iter().map(
                    |x| format!("{}<{}>", base_ident, x.to_token_stream())
                  ).collect();
              }
            });

            if new_idents.len() == 0 {
              new_idents.push(base_ident);
            }

            for ident in new_idents {
              let count = *types.get(&ident).unwrap_or(&0_usize);
              types.insert(ident, count + 1);
            }
          }
        }),
      Fields::Unnamed(_fields) => panic!(
        "Derive(FieldCountByType) only applicable to named structs"),
      Fields::Unit => (),
    },
    _ => panic!("Derive(FieldCountByType) only applicable to structs"),
  }

  let keys: Vec<Type> = types.keys()
    .map(|key| {
      let key : proc_macro2::TokenStream = key.parse().unwrap();
      parse_quote!(#key)
    })
    .collect();
  let values = types.values();

  let res = TokenStream::from(quote! {
    #(
      impl field_count::FieldCountByType<#keys> for #name {
        fn field_count_by_type(&self) -> usize {#values}
      }
    )*
  });

  res
}
