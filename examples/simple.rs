use tlv::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    #[tlv(tag = 1)]
    pub id: u32,

    #[tlv(tag = 2)]
    pub payload: Vec<u8>,

    #[tlv(tag = 3)]
    pub description: Option<String>,
}

fn main() {}
