use quote::ToTokens;
use syn::{punctuated::Punctuated, LitInt};

pub(crate) fn parse_tlv_fields_attributes(
    fields: &Punctuated<syn::Field, syn::Token![,]>,
) -> Result<Vec<TlvFieldAttributes>, syn::Error> {
    fields
        .iter()
        .flat_map(|field| field.attrs.to_owned())
        .filter(|attr| attr.path.is_ident("tlv"))
        .map(TryFrom::try_from)
        .collect()
}

/// Attributes for a TLV field.
///
/// Specified using the `#[tlv()]` macro attribute
pub struct TlvFieldAttributes {
    /// The `tag` of the field
    pub tag: LitInt,
}

impl TlvFieldAttributes {
    pub fn new(tag: LitInt) -> Self {
        Self { tag }
    }
}

impl TryFrom<syn::Attribute> for TlvFieldAttributes {
    type Error = syn::Error;

    fn try_from(value: syn::Attribute) -> Result<Self, Self::Error> {
        let meta = value.parse_meta().unwrap();
        let list = match meta {
            syn::Meta::List(meta) => meta,
            _ => {
                return Err(syn::Error::new_spanned(
                    &meta,
                    "Attribute must be like #[tlv(tag = 1)]",
                ))
            }
        };

        let (key, value) = match list.nested.first().unwrap() {
            syn::NestedMeta::Meta(syn::Meta::NameValue(syn::MetaNameValue {
                path,
                lit: syn::Lit::Int(lit),
                ..
            })) => (path, lit),
            _ => {
                return Err(syn::Error::new_spanned(
                    &list,
                    format!(
                        "Invalid attribute, shoudld be tag = 1, got: {}",
                        list.nested.first().unwrap().to_token_stream()
                    ),
                ))
            }
        };

        if key.is_ident("tag") {
            Ok(TlvFieldAttributes::new(value.clone()))
        } else {
            Err(syn::Error::new_spanned(
                &key,
                "Invalid attribute: only 'tag' is valid",
            ))
        }
    }
}
