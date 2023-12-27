use std::convert::TryFrom;

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse::{discouraged::Speculative, Parse},
    parse_quote,
    spanned::Spanned,
    Error, Field, Lifetime, LitInt, LitStr, Meta, MetaList, Result, Token, Type,
};

use crate::utils;

pub enum FieldMapping {
    /// An internal field that is mapped 1:1 to an API field
    OneToOne(OneToOne),

    /// An API field that has no corresponding internal field.
    ///
    /// These get initialized to [`Default::default`] during deserialization.
    NoIndex { field: Ident },
}

pub enum LitIndex {
    Int(LitInt),
    Str(LitStr),
}

pub struct OneToOne {
    /// The index of this field in the internal data format
    pub index: LitIndex,

    /// The API field to which this [`InternalField`] is mapped
    pub field: Ident,

    /// The type of the API field associated with this internal field
    pub api_type: Type,

    /// Attributes to pass through as #[serde(...)] attributes in the internal structures
    pub passthrough: Vec<TokenStream>,
}

impl OneToOne {
    fn ser_type(&self, lifetime: &Lifetime) -> Type {
        let api_type = &self.api_type;

        parse_quote! {
            <#api_type as crate::serde::InternalProxy>::SerializeProxy<#lifetime>
        }
    }

    fn de_type(&self) -> Type {
        let api_type = &self.api_type;
        parse_quote! {
            <#api_type as crate::serde::InternalProxy>::DeserializeProxy
        }
    }

    fn index(&self) -> String {
        match &self.index {
            LitIndex::Int(lit_int) => lit_int.base10_digits().to_string(),
            LitIndex::Str(lit_str) => lit_str.value(),
        }
    }

    fn internal_name(&self) -> Ident {
        format_ident!("index_{}", self.index())
    }

    fn field_tokens(&self, ty: Type) -> proc_macro2::TokenStream {
        let serde_name = self.index();
        let field_name = self.internal_name();
        let passthrough = &self.passthrough;

        if utils::type_contains_lifetime(&ty) {
            quote! {
                #[serde(rename = #serde_name)]
                #[serde(borrow)]
                #(
                    #[serde(#passthrough)]
                )*
                pub #field_name: #ty,
            }
        } else {
            quote! {
                #[serde(rename = #serde_name)]
                #(
                    #[serde(#passthrough)]
                )*
                pub #field_name: #ty,
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
        let field_name = self.internal_name();
        let field = &self.field;

        quote! {
            #field_name: self.#field.to_serialize_proxy(),
        }
    }

    pub fn deserialize(&self) -> proc_macro2::TokenStream {
        let field_name = self.internal_name();
        let field = &self.field;
        let api_type = &self.api_type;

        quote! {
            #field: <#api_type>::from_deserialize_proxy(internal.#field_name),
        }
    }
}

impl FieldMapping {
    pub fn ser_field_tokens(&self, lifetime: &Lifetime) -> TokenStream {
        match self {
            FieldMapping::OneToOne(inner) => inner.ser_field_tokens(lifetime),
            FieldMapping::NoIndex { .. } => quote!(),
        }
    }

    pub fn de_field_tokens(&self) -> TokenStream {
        match self {
            FieldMapping::OneToOne(inner) => inner.de_field_tokens(),
            FieldMapping::NoIndex { .. } => quote!(),
        }
    }

    pub fn serialize(&self) -> TokenStream {
        match self {
            FieldMapping::OneToOne(inner) => inner.serialize(),
            FieldMapping::NoIndex { .. } => quote!(),
        }
    }

    pub fn deserialize(&self) -> TokenStream {
        match self {
            FieldMapping::OneToOne(inner) => inner.deserialize(),
            FieldMapping::NoIndex { field } => quote! {
                #field: Default::default(),
            },
        }
    }
}

#[derive(Default)]
enum FieldMappingBuilder {
    #[default]
    Initial,
    OneToOne {
        index: Option<LitIndex>,
        passthrough: Vec<TokenStream>,
    },
    NoIndex,
}

impl FieldMappingBuilder {
    fn with_index(&mut self, index: LitIndex) -> bool {
        match std::mem::take(self) {
            FieldMappingBuilder::Initial => {
                *self = FieldMappingBuilder::OneToOne {
                    index: Some(index),
                    passthrough: Vec::new(),
                }
            },
            FieldMappingBuilder::OneToOne { index: None, passthrough } => {
                *self = FieldMappingBuilder::OneToOne {
                    index: Some(index),
                    passthrough,
                }
            },
            _ => return false,
        }
        true
    }

    fn no_index(&mut self) -> bool {
        match std::mem::take(self) {
            FieldMappingBuilder::Initial => *self = FieldMappingBuilder::NoIndex,
            _ => return false,
        }
        true
    }

    fn with_passthrough(&mut self, tokens: TokenStream) -> bool {
        match std::mem::take(self) {
            FieldMappingBuilder::Initial => {
                *self = FieldMappingBuilder::OneToOne {
                    index: None,
                    passthrough: vec![tokens],
                }
            },
            FieldMappingBuilder::OneToOne { index, mut passthrough } => {
                passthrough.push(tokens);
                *self = FieldMappingBuilder::OneToOne { index, passthrough }
            },
            FieldMappingBuilder::NoIndex => return false,
        }
        true
    }
}

impl TryFrom<Field> for FieldMapping {
    type Error = Error;

    fn try_from(field: Field) -> Result<Self> {
        let span = field.span();
        let api_type = field.ty;

        let mut builder = FieldMappingBuilder::Initial;

        for attr in field.attrs {
            let Meta::List(MetaList { path, tokens, .. }) = attr.meta else {
                continue;
            };

            if path.segments.len() != 1 || path.segments[0].ident != "dash" {
                continue;
            }

            let build_success = match syn::parse2::<DashAttribute>(tokens)? {
                DashAttribute::Index(idx) => builder.with_index(idx),
                DashAttribute::PassthroughToSerde(tokens) => builder.with_passthrough(tokens),
                DashAttribute::NoIndex => builder.no_index(),
            };

            if !build_success {
                return Err(Error::new(span, "unexpected #[dash(...)] attribute"));
            }
        }

        let field = field.ident.unwrap();

        match builder {
            FieldMappingBuilder::Initial => Err(Error::new_spanned(field, "missing #[dash(...)] attribute")),
            FieldMappingBuilder::OneToOne {
                index: Some(index),
                passthrough,
            } => Ok(FieldMapping::OneToOne(OneToOne {
                index,
                field,
                api_type,
                passthrough,
            })),
            FieldMappingBuilder::OneToOne { index: None, .. } => Err(Error::new_spanned(field, "missing #[dash(index = ...)] attribute")),
            FieldMappingBuilder::NoIndex => Ok(FieldMapping::NoIndex { field }),
        }
    }
}

enum DashAttribute {
    Index(LitIndex),
    NoIndex,
    PassthroughToSerde(TokenStream),
}

impl Parse for DashAttribute {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let fork = input.fork();

        if let Ok(key) = fork.parse::<Ident>() {
            if key == "no_index" {
                input.advance_to(&fork);

                return Ok(DashAttribute::NoIndex);
            }
            if key == "index" {
                let _ = fork.parse::<Token![=]>()?;
                let lookahead = fork.lookahead1();

                let lit = if lookahead.peek(LitInt) {
                    LitIndex::Int(fork.parse()?)
                } else if lookahead.peek(LitStr) {
                    LitIndex::Str(fork.parse()?)
                } else {
                    return Err(lookahead.error());
                };

                input.advance_to(&fork);

                return Ok(DashAttribute::Index(lit));
            }
        }

        input.parse().map(DashAttribute::PassthroughToSerde)
    }
}
