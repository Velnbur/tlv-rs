use syn::{GenericArgument, Type, TypePath};

pub(crate) fn ty_is_vec_u8(ty: &Type) -> bool {
    let Type::Path(syn::TypePath { path, .. }) = ty else {
        return false;
    };

    let Some(segment) = path.segments.first() else {
        return false;
    };

    if segment.ident.to_string() != "Vec" {
        return false;
    }

    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
        return false;
    };

    let Some(GenericArgument::Type(Type::Path(TypePath { path, .. }))) = args.args.first() else {
        return false;
    };

    path.is_ident("u8")
}

pub(crate) fn ty_is_option(ty: &Type) -> bool {
    let Type::Path(syn::TypePath { path, .. }) = ty else {
        return false;
    };

    let segment = path.segments.last().unwrap();
    let ident = &segment.ident;

    ident.to_string() == "Option"
}
