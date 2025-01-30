extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn specled(attr: TokenStream, input: TokenStream) -> proc_macro::TokenStream {

    let specs = parse_macro_input!(attr as Ident);
    let item = parse_macro_input!(input as ItemStruct);
    let struct_identifier = item.ident.clone();

    if item.fields.len() != 1 {
        panic!("To create a struct instance with a specification, the struct must have exactly one field");
    }

    let field_type = item.fields.iter().next().map(|field| {
        field.ty.clone().into_token_stream()
    });

    let original_item = item.into_token_stream();
    let output = quote! {
        #original_item

        #[doc="Macro generated factory trait implementation"]
        impl specler::prelude::ValueObjectFactory<#field_type, #struct_identifier, #specs> for #struct_identifier {
            fn create(pre_validated_input: impl Into<#field_type>) -> Result<#struct_identifier, SpecError> {
                let input: #field_type = pre_validated_input.into();
                let validated_input: specler::prelude::Validated<#field_type, #specs> = input.into();
                match validated_input{
                    specler::prelude::Validated::Valid { value, _spec } => Ok(#struct_identifier(value)),
                    specler::prelude::Validated::Invalid { error, _spec } => Err(error.named(stringify!(#struct_identifier))),
                }
            }
        }

        impl specler::prelude::ValueObject<#field_type> for #struct_identifier {
            fn value(&self) -> &#field_type {
                &self.0
            }
        }
    };

    output.into()
}