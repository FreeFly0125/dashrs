use std::convert::TryFrom;

use field::InternalField;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::ToTokens;
use struct_gen::InternalStruct;
use syn::{spanned::Spanned, Data, DataStruct, DeriveInput, Error, Fields, Result, parse_macro_input};

mod field;
mod struct_gen;
mod utils;

#[proc_macro_derive(Dash, attributes(dash))]
pub fn derive_dash(ts: TokenStream) -> TokenStream {
    let input = parse_macro_input!(ts as DeriveInput);
    expand_dash_derive(input)
        .map(|is| is.to_token_stream())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn expand_dash_derive(input: DeriveInput) -> Result<InternalStruct> {
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = input;

    let Data::Struct(DataStruct { fields, .. }) = data else {
        return Err(Error::new(Span::call_site(), "#[derive(Dash)] only support structs"));
    };

    let Fields::Named(fields_named) = fields else {
        return Err(Error::new(fields.span(), "#[derive(Dash) only supports structs with named fields"));
    };

    let primary_lifetime = utils::find_unique_lifetime(&generics)?;

    fields_named
        .named
        .into_iter()
        .map(InternalField::try_from)
        .collect::<Result<Vec<_>>>()
        .map(|fields| InternalStruct {
            name: ident,
            fields,
            generics,
            lifetime: primary_lifetime,
        })
}
