pub use crate::deser::utils::{deserialize, deserialize_bytes};
pub use crate::deser::{extract_raw, Deserialize, RawField};

pub use crate::ser::utils::{serialize, serialize_bytes};
pub use crate::ser::Serialize;

mod deser;
pub mod impls;
mod ser;

#[cfg(feature = "derive")]
pub use tlv_derive::*;
