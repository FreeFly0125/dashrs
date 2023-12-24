use std::convert::TryFrom;

use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::parse::discouraged::Speculative;
use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::{parse_quote, Error, Field, Lifetime, LitInt, Meta, MetaList, Result, Token, Type};

use crate::utils;

pub enum InternalField {
    /// An internal field that is mapped 1:1 to an API field
    OneToOne {
        /// The index of this field in the internal data format
        index: LitInt,

        /// The API field to which this [`InternalField`] is mapped
        field: Ident,

        /// The type of the API field associated with this internal field
        api_type: Type,

        passthrough: Vec<TokenStream>,
    },
}

impl InternalField {
    fn internal_name(&self) -> Ident {
        format_ident!("index_{}", self.numeric_index())
    }

    fn numeric_index(&self) -> &str {
        match self {
            InternalField::OneToOne { index, .. } => index.base10_digits(),
        }
    }

    fn serde_passthrough(&self) -> &[TokenStream] {
        match self {
            InternalField::OneToOne { passthrough, .. } => passthrough.as_ref(),
        }
    }

    fn ser_type(&self, lifetime: &Lifetime) -> Type {
        match self {
            InternalField::OneToOne { api_type, .. } => parse_quote! {
                <#api_type as crate::serde::InternalProxy>::SerializeProxy<#lifetime>
            },
        }
    }

    fn de_type(&self) -> Type {
        match self {
            InternalField::OneToOne { api_type, .. } => parse_quote! {
                <#api_type as crate::serde::InternalProxy>::DeserializeProxy
            },
        }
    }

    fn field_tokens(&self, ty: Type) -> proc_macro2::TokenStream {
        let field_name = self.internal_name();
        let serde_name = self.numeric_index();
        let passthrough = self.serde_passthrough();

        if utils::type_contains_lifetime(&ty) {
            quote! {
                #[serde(rename = #serde_name)]
                #[serde(borrow)]
                #(
                    #[serde(#passthrough)]
                )*
                pub #field_name: #ty
            }
        } else {
            quote! {
                #[serde(rename = #serde_name)]
                #(
                    #[serde(#passthrough)]
                )*
                pub #field_name: #ty
            }
        }
    }

    pub fn ser_field_tokens(&self, lifetime: &Lifetime) -> proc_macro2::TokenStream {
        self.field_tokens(self.ser_type(lifetime))
    }

    pub fn de_field_tokens(&self) -> proc_macro2::TokenStream {
        self.field_tokens(self.de_type())
    }

    pub fn serialize(&self) -> proc_macro2::TokenStream {
        match self {
            InternalField::OneToOne { field, .. } => {
                let field_name = self.internal_name();

                quote! {
                    #field_name: self.#field.to_serialize_proxy()
                }
            },
        }
    }

    pub fn deserialize(&self) -> proc_macro2::TokenStream {
        match self {
            InternalField::OneToOne { field, api_type, .. } => {
                let field_name = self.internal_name();

                quote! {
                    #field: <#api_type>::from_deserialize_proxy(internal.#field_name)
                }
            },
        }
    }
}

impl TryFrom<Field> for InternalField {
    type Error = Error;

    fn try_from(field: Field) -> Result<Self> {
        let span = field.ident.span();
        let api_type = field.ty;

        let mut passthrough = Vec::new();
        let mut index = None;

        for attr in field.attrs {
            let Meta::List(MetaList { path, tokens, .. }) = attr.meta else {
                continue;
            };

            if path.segments.len() != 1 || path.segments[0].ident != "dash" {
                continue;
            }

            match syn::parse2::<DashAttribute>(tokens)? {
                DashAttribute::Index(idx) => index = Some(idx),
                DashAttribute::PassthroughToSerde(tokens) => passthrough.push(tokens),
            }
        }

        match index {
            Some(index) => Ok(InternalField::OneToOne {
                index,
                field: field.ident.unwrap(),
                api_type,
                passthrough,
            }),
            None => Err(Error::new(span, "Field missing index mapping")),
        }
    }
}

enum DashAttribute {
    Index(LitInt),
    PassthroughToSerde(TokenStream),
}

impl Parse for DashAttribute {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let fork = input.fork();

        let maybe_knowable = fork.parse::<Ident>().and_then(|key| {
            let _ = fork.parse::<Token![=]>()?;
            Ok(key)
        });
        match maybe_knowable {
            Ok(key) if key == "index" => {
                let lit_int = fork.parse::<LitInt>()?;
                input.advance_to(&fork);
                Ok(DashAttribute::Index(lit_int))
            },
            _ => Ok(DashAttribute::PassthroughToSerde(input.parse()?)),
        }
    }
}
