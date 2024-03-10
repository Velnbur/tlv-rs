extern crate proc_macro;

use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput};

mod attributes;
mod der;
mod ser;
mod utils;

/// Derive macro for the `Serialize` trait.
///
/// This macro has some special encodings for certain types. For `Vec<u8>` there is no
/// need to iterativly serialize each byte, so we use `serialize_bytes` instead.
///
/// # Example
///
/// ```
/// use tlv::Serialize;
///
/// #[derive(Serialize)]
/// struct MyStruct {
///    #[tlv(tag = 1)]
///    field1: u8,
///    #[tlv(tag = 2)]
///    field2: String,
/// }
/// ```
///
/// This will generate an implementation of the `Serialize` trait for `MyStruct`.
///
/// The generated code will look like this:
///
/// ```
/// struct MyStruct {
///   field1: u8,
///   field2: String,
/// }
///
/// impl tlv::Serialize for MyStruct {
///    fn serialize<W>(&self, writer: &mut W) -> std::io::Result<usize>
///    where
///        W: std::io::Write,
///    {
///        let mut len = 0;
///
///        len += ::tlv::Serialize::serialize(&(1 as u16), writer)?;
///        len += ::tlv::Serialize::serialize(
///            &::tlv::Serialize::serialized_length(&self.field1),
///            writer,
///        )?;
///        len += self.field1.serialize(writer)?;
///
///        len += ::tlv::Serialize::serialize(&(2 as u16), writer)?;
///        len += ::tlv::Serialize::serialize(
///            &::tlv::Serialize::serialized_length(&self.field2),
///            writer,
///        )?;
///        len += self.field2.serialize(writer)?;
///
///        Ok(len)
///    }
/// }
/// ```
#[proc_macro_derive(Serialize, attributes(tlv))]
pub fn tlv_serialize_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    match ser::tlv_serialize_derive_impl(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Derive macro for the `Deserialize` trait.
///
/// This macro has special decoding for certain types. For `Vec<u8>` there is no
/// need to iterativly deserialize each byte, so we use `deserialize_bytes` instead,
/// for optional fields we use the `Option` type, and won't return an error if the
/// field is not present.
///
/// # Example
///
/// ```
/// use tlv::Deserialize;
///
/// #[derive(Deserialize)]
/// struct MyStruct {
///   #[tlv(tag = 1)]
///   field1: u8,
///   #[tlv(tag = 2)]
///   field2: String,
/// }
/// ```
///
/// This will generate an implementation of the `Deserialize` trait for `MyStruct`.
///
/// The generated code will look like this:
///
/// ```
/// struct MyStruct {
///   field1: u8,
///   field2: String,
/// }
///
/// impl tlv::Deserialize for MyStruct {
///   fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
///     const EXPECTED_IDS: [u8; 2] = [1, 2];
///
///     let fields = ::tlv::extract_raw(reader, EXPECTED_IDS)?;
///
///     let field1 = fields
///         .get(&1)
///         .map(|field| ::tlv::Deserialize::deserialize(&mut std::io::Cursor::new(field.value.as_slice())))
///         .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "missing field1"))??;
///
///     let field2 = fields
///         .get(&2)
///         .map(|field| ::tlv::Deserialize::deserialize(&mut std::io::Cursor::new(field.value.as_slice())))
///         .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "missing field2"))??;
///
///    Ok(Self {
///     field1,
///     field2,
///    })
///   }
/// }
/// ```
#[proc_macro_derive(Deserialize, attributes(tlv))]
pub fn tlv_deserialize_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    match der::tlv_deserialize_derive_impl(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
