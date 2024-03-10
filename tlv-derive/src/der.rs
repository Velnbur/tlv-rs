use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{punctuated::Punctuated, spanned::Spanned, Data, DeriveInput, Field, Fields, Token};

use crate::{
    attributes::{parse_tlv_fields_attributes, TlvFieldAttributes},
    utils::{ty_is_option, ty_is_vec_u8},
};

pub fn tlv_deserialize_derive_impl(input: DeriveInput) -> Result<TokenStream, syn::Error> {
    // Get the name of the struct
    let struct_name = &input.ident;

    // Get the fields of the struct
    match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => impl_for_struct(struct_name, fields.named.clone()),
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

fn impl_for_struct(
    struct_name: &Ident,
    fields: Punctuated<Field, Token![,]>,
) -> Result<TokenStream, syn::Error> {
    let attributes = parse_tlv_fields_attributes(&fields)?;

    // Generate code for deserialization
    let mut deserialization_code = Vec::new();
    let mut expected_tags = Vec::new();
    let mut field_extraction = Vec::new();

    for (field, attributes) in fields.iter().zip(attributes.iter()) {
        let tag = &attributes.tag;
        let ident = &field.ident;

        let field_deserialization = create_deserializer_for_field(field, attributes)?;

        expected_tags.push(tag);
        deserialization_code.push(quote! {
            #field_deserialization
        });
        field_extraction.push(quote! {
            #ident,
        });
    }

    let expected_tags_len = expected_tags.len();

    let expected_tags = quote! { [ #( #expected_tags ),* ] };

    // Generate the code for the implementation
    let gen = quote! {
        impl ::tlv::Deserialize for #struct_name {
            fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
                const EXPECTED_TAGS: [u8; #expected_tags_len] = #expected_tags;

                let fields = ::tlv::extract_raw(reader, EXPECTED_TAGS)?;

                #(#deserialization_code)*

                Ok(Self {
                    #(#field_extraction)*
                })
            }
        }
    };

    // Return the generated implementation
    Ok(gen.into())
}

fn create_deserializer_for_field(
    field: &Field,
    attribute: &TlvFieldAttributes,
) -> Result<TokenStream, syn::Error> {
    let tag = &attribute.tag;
    let name = &field
        .ident
        .as_ref()
        .ok_or_else(|| syn::Error::new(field.span(), "missing field name"))?;
    let field_ty = &field.ty;

    let kind = DeserializerKind::from(field);

    match kind {
        DeserializerKind::Bytes => gen_bytes_deserializer(tag, name, field_ty),
        DeserializerKind::Optional => gen_optional_deserializer(tag, name, field_ty),
        DeserializerKind::Regular => gen_regular_deserializer(tag, name, field_ty),
    }
}

fn gen_bytes_deserializer(
    tag: &syn::LitInt,
    name: &Ident,
    field_ty: &syn::Type,
) -> Result<TokenStream, syn::Error> {
    Ok(quote! {
        let #name: #field_ty = fields
            .get(&#tag)
            .map(|field| ::tlv::deserialize_bytes(
                &mut ::std::io::Cursor::new(field.value.as_slice())
            ))
            .transpose()?
            .unwrap_or_else(|| Vec::new());
    }
    .into())
}

fn gen_optional_deserializer(
    tag: &syn::LitInt,
    name: &Ident,
    field_ty: &syn::Type,
) -> Result<TokenStream, syn::Error> {
    Ok(quote! {
        let #name: #field_ty = fields
            .get(&#tag)
            .map(|field| ::tlv::Deserialize::deserialize(
                &mut std::io::Cursor::new(field.value.as_slice())
            ))
            .transpose()?;
    }
    .into())
}

fn gen_regular_deserializer(
    tag: &syn::LitInt,
    name: &Ident,
    field_ty: &syn::Type,
) -> Result<TokenStream, syn::Error> {
    let error_msg = format!("missing {} field", name);

    Ok(quote! {
        let #name: #field_ty = fields
            .get(&#tag)
            .map(|field| ::tlv::Deserialize::deserialize(
                &mut std::io::Cursor::new(field.value.as_slice())
            ))
            .ok_or_else(|| std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                #error_msg,
            ))??;
    }
    .into())
}

enum DeserializerKind {
    /// The field is a Vec<u8>, and we need to use the specialized
    /// `deserialize_bytes` function.
    Bytes,

    /// The field is optional, and we need to use the `Option` type
    /// to deserialize it and don't need the return error if the field
    /// is not present.
    Optional,

    /// The field is a regular field and we can use the `Deserialize`
    /// trait to deserialize it.
    Regular,
}

impl From<&Field> for DeserializerKind {
    fn from(field: &Field) -> Self {
        let ty = &field.ty;

        if ty_is_vec_u8(ty) {
            DeserializerKind::Bytes
        } else if ty_is_option(ty) {
            DeserializerKind::Optional
        } else {
            DeserializerKind::Regular
        }
    }
}
