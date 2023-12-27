use syn::{spanned::Spanned, Error, Generics, LifetimeParam, Result, Type};

/// If the given [`Generics`] contain a unique lifetime, return it. If there are no lifetimes,
/// return a `'static` lifetime. Otherwise, return a spanned error indicating either a lack of
/// lifetimes, or too many lifetimes
pub fn find_unique_lifetime(generics: &Generics) -> Result<Option<LifetimeParam>> {
    let mut lifetime_iter = generics.lifetimes();
    let first_lifetime = lifetime_iter.next().map(Clone::clone);

    if let Some(lifetime_param) = lifetime_iter.next() {
        return Err(Error::new(lifetime_param.span(), "Expected exactly one lifetime, found multiple"));
    }

    Ok(first_lifetime)
}

pub fn type_contains_lifetime(ty: &Type) -> bool {
    match ty {
        Type::Array(_) => todo!(),
        Type::BareFn(_) => todo!(),
        Type::Group(_) => todo!(),
        Type::ImplTrait(_) => todo!(),
        Type::Infer(_) => todo!(),
        Type::Macro(_) => todo!(),
        Type::Never(_) => todo!(),
        Type::Paren(_) => todo!(),
        Type::Path(type_path) => {
            type_path.path.segments.iter().any(|segment| match &segment.arguments {
                syn::PathArguments::None => false,
                syn::PathArguments::AngleBracketed(generic_type_parameters) => {
                    generic_type_parameters.args.iter().any(|generic| match generic {
                        syn::GenericArgument::Lifetime(_) => true,
                        syn::GenericArgument::Type(ty) => type_contains_lifetime(ty),
                        syn::GenericArgument::Const(_) => false,
                        syn::GenericArgument::AssocType(assoc_ty) => type_contains_lifetime(&assoc_ty.ty),
                        syn::GenericArgument::AssocConst(_) => false,
                        syn::GenericArgument::Constraint(_) => todo!(),
                        _ => todo!(),
                    })
                },
                syn::PathArguments::Parenthesized(_) => todo!(),
            }) || type_path
                .qself
                .as_ref()
                .map(|qself| type_contains_lifetime(&qself.ty))
                .unwrap_or(false)
        },
        Type::Ptr(_) => todo!(),
        Type::Reference(reference) => reference.lifetime.is_some(),
        Type::Slice(_) => todo!(),
        Type::TraitObject(_) => todo!(),
        Type::Tuple(_) => todo!(),
        Type::Verbatim(_) => todo!(),
        _ => todo!(),
    }
}
