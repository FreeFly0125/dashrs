use proc_macro2::{Ident, Span};
use quote::{format_ident, quote, ToTokens};
use syn::{Generics, Lifetime, LifetimeParam};

use crate::field::FieldMapping;

pub struct InternalStruct {
    /// Name of the API-version of this struct
    pub name: Ident,
    pub fields: Vec<FieldMapping>,
    pub generics: Generics,
    /// The unique lifetime of the struct for which we are deriving `Dash`, if it exists.
    pub lifetime: Option<LifetimeParam>,
}

impl InternalStruct {
    fn serialize_struct_name(&self) -> Ident {
        format_ident!("Internal{}Ser", self.name)
    }

    fn deserialize_struct_name(&self) -> Ident {
        format_ident!("Internal{}De", self.name)
    }

    fn ser_struct(&self) -> proc_macro2::TokenStream {
        let name = self.serialize_struct_name();
        let static_lifetime = Lifetime::new("'static", Span::call_site());
        let lifetime = match self.lifetime {
            Some(ref lifetime) => &lifetime.lifetime,
            None => &static_lifetime,
        };
        let fields = self.fields.iter().map(|ifield| ifield.ser_field_tokens(lifetime));
        let generics = &self.generics;

        quote! {
            #[derive(Serialize)]
            struct #name#generics {
                #(#fields)*
            }
        }
    }

    fn de_struct(&self) -> proc_macro2::TokenStream {
        let name = self.deserialize_struct_name();
        let fields = self.fields.iter().map(|ifield| ifield.de_field_tokens());
        let generics = &self.generics;

        quote! {
            #[derive(Deserialize)]
            struct #name#generics {
                #(#fields)*
            }
        }
    }

    fn serialize_implementation(&self) -> proc_macro2::TokenStream {
        // assume a `Serializer` is in scope, named serializer
        let serialize_struct = self.serialize_struct_name();
        let initializers = self.fields.iter().map(|ifield| ifield.serialize());

        quote! {
            let internal = #serialize_struct {
                #(#initializers)*
            };
            internal.serialize(serializer)
        }
    }

    fn deserialize_implementation(&self) -> proc_macro2::TokenStream {
        // assume a `Deserializer` is in scope, named deserializer
        let deserialize_struct = self.deserialize_struct_name();
        let api_struct = &self.name;
        let initializers = self.fields.iter().map(|ifield| ifield.deserialize());

        quote! {
            let internal = #deserialize_struct::deserialize(deserializer)?;

            Ok(#api_struct {
                #(#initializers)*
            })
        }
    }
}

impl ToTokens for InternalStruct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ser_struct = self.ser_struct();
        let de_struct = self.de_struct();
        let name = &self.name;

        let artificial_lifetime = Lifetime::new("'__dash", Span::call_site());
        let existing_params = &self.generics.params;
        let (generic_arg_list, lifetime) = match self.lifetime {
            Some(ref lifetime) => (quote! {<#existing_params>}, &lifetime.lifetime),
            None => (quote! {<#artificial_lifetime,#existing_params>}, &artificial_lifetime),
        };
        let where_clause = &self.generics.where_clause;

        let deserialize_impl = self.deserialize_implementation();
        let serialize_impl = self.serialize_implementation();

        tokens.extend(quote! {
            const _: () = {
                use serde::{Serializer, Deserializer};
                use crate::serde::Dash;
                use crate::serde::InternalProxy;

                #ser_struct
                #de_struct

                impl#generic_arg_list Dash<#lifetime> for #name<#existing_params>
                    #where_clause
                {
                    fn dash_deserialize<D: Deserializer<#lifetime>>(deserializer: D) -> Result<Self, D::Error> {
                        #deserialize_impl
                    }

                    fn dash_serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                        #serialize_impl
                    }
                }
            };
        })
    }
}
