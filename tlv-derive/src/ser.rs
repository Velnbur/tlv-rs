use proc_macro2::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, DeriveInput};

use crate::{attributes::parse_tlv_fields_attributes, utils::ty_is_vec_u8};

pub(crate) fn tlv_serialize_derive_impl(input: DeriveInput) -> Result<TokenStream, syn::Error> {
    // Get the name of the struct
    let struct_name = &input.ident;

    // Get the fields of the struct
    match input.data {
        syn::Data::Struct(ref data) => match data.fields {
            syn::Fields::Named(ref fields) => impl_for_struct(struct_name, &fields.named),
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

fn impl_for_struct(
    struct_name: &syn::Ident,
    fields: &Punctuated<syn::Field, syn::Token![,]>,
) -> Result<TokenStream, syn::Error> {
    let attributes = parse_tlv_fields_attributes(fields)?;

    let field_serialize_function = fields
        .iter()
        .zip(attributes.iter())
        .map(|(field, attributes)| {
            let serializer = create_serializer_for_field(field);

            let id = &attributes.tag;
            let serialize_type = quote! {
                len += ::tlv::Serialize::serialize(&(#id as u16), writer)?;
            };

            Ok(quote! {
                // Serialize type
                #serialize_type
                // Serialize value and length
                #serializer
            })
        })
        .collect::<Result<Vec<_>, syn::Error>>()?;

    // Generate the code for the implementation
    let gen = quote! {
        #[automatically_derived]
        impl ::tlv::Serialize for #struct_name {
            fn serialize<W>(&self, writer: &mut W) -> ::std::io::Result<usize>
            where
                W: ::std::io::Write
            {
                let mut len = 0;

                #(#field_serialize_function)*

                Ok(len)
            }
        }
    };

    // Return the generated implementation
    Ok(gen.into())
}

fn create_serializer_for_field(field: &syn::Field) -> TokenStream {
    let ty = field.ty.to_owned();
    let name = field.ident.as_ref().unwrap();

    // Check if the field is a Vec<u8> and use the specialized function
    // for serializing bytes. Otherwise, use the generic Serialize trait
    let is_bytes = ty_is_vec_u8(&ty);

    if is_bytes {
        // `serialize_bytes` encodes the length of the vec of bytes as well
        quote! {
            len += ::tlv::serialize_bytes(&self.#name, writer)?;
        }
    } else {
        quote! {
            // serialize length
            len += ::tlv::Serialize::serialize(
                &::tlv::Serialize::serialized_length(&self.#name),
                writer,
            )?;
            // serialize value
            len += ::tlv::Serialize::serialize(&self.#name, writer)?;
        }
    }
}
