use std::convert::TryFrom;

use field::FieldMapping;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::ToTokens;
use struct_gen::InternalStruct;
use syn::{parse_macro_input, spanned::Spanned, Data, DataStruct, DeriveInput, Error, Fields, Result};

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
    let DeriveInput { ident, generics, data, .. } = input;

    let Data::Struct(DataStruct { fields, .. }) = data else {
        return Err(Error::new(Span::call_site(), "#[derive(Dash)] only support structs"));
    };

    let Fields::Named(fields_named) = fields else {
        return Err(Error::new(fields.span(), "#[derive(Dash) only supports structs with named fields"));
    };

    let primary_lifetime = utils::find_unique_lifetime(&generics)?;

    #[allow(clippy::manual_try_fold)] //
    let fields = fields_named
        .named
        .into_iter()
        .map(FieldMapping::try_from)
        .fold(Ok(Vec::new()), |acc, res| match (acc, res) {
            (Ok(mut ifields), Ok(ifield)) => {
                ifields.push(ifield);
                Ok(ifields)
            },
            (Ok(_), Err(err)) | (Err(err), Ok(_)) => Err(err),
            (Err(mut err), Err(err2)) => {
                err.combine(err2);
                Err(err)
            },
        })?;

    Ok(InternalStruct {
        name: ident,
        fields,
        generics,
        lifetime: primary_lifetime,
    })
}
