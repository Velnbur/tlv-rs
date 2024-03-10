# `tlv-rs`

Implementation of TLV encding written in rust with ability to implement
encoding/decoding with deriviable macros.

```rust
#[derive(tlv::Serialize, tlv::Deserialize)]
pub struct Message {
    #[tlv(tag = 1)]
    pub id: u32,

    #[tlv(tag = 2)]
    pub payload: Vec<u8>,

    #[tlv(tag = 3)]
    pub description: Option<String>,
}
```
