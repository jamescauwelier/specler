extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn value_object(attr: TokenStream, input: TokenStream) -> proc_macro::TokenStream {

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
        impl ValueObjectFactory<#field_type, #struct_identifier, #specs> for #struct_identifier {
            fn create(pre_validated_input: impl Into<#field_type>) -> Result<#struct_identifier, SpecError> {
                let input: #field_type = pre_validated_input.into();
                let validated_input: Validated<#field_type, #specs> = input.into();
                match validated_input{
                    Validated::Valid { value, _spec } => Ok(#struct_identifier(value)),
                    Validated::Invalid { error, _spec } => Err(error.named(stringify!(#struct_identifier))),
                }
            }
        }

        impl ValueObject<#field_type> for #struct_identifier {
            fn value(&self) -> &#field_type {
                &self.0
            }
        }
    };

    output.into()
}

#[proc_macro_attribute]
pub fn arbitrary_spec_values(attr: TokenStream, input: TokenStream) -> proc_macro::TokenStream {

    let specs = parse_macro_input!(attr as Ident);
    let item = parse_macro_input!(input as ItemStruct);
    let struct_identifier = item.ident.clone();

    if item.fields.len() != 1 {
        panic!("To create a struct instance with a specification, the struct must have exactly one field");
    }

    // let field_type = item.fields.iter().next().map(|field| {
    //     field.ty.clone().into_token_stream()
    // });

    let original_item = item.into_token_stream();
    let output = quote! {
        #original_item

        #[doc="Generates arbitrary values for a spec, but requires the implementation of "]
        #[cfg(test)]
        impl Arbitrary for #struct_identifier {
            type Parameters = ();

            fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
                #specs::arbitrary()
                    .prop_map(|s| #struct_identifier::create(s).unwrap)
                    .boxed()
            }

            type Strategy = BoxedStrategy<#struct_identifier>;
        }
    };

    output.into()
}