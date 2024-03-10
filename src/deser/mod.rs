use std::collections::BTreeMap;

pub(crate) mod utils;

pub trait Deserialize {
    fn deserialize<R>(reader: &mut R) -> std::io::Result<Self>
    where
        R: std::io::Read,
        Self: Sized;
}

pub struct RawField {
    pub id: u8,
    pub len: u16,
    pub value: Vec<u8>,
}

impl RawField {
    pub fn new(id: u8, len: u16, value: Vec<u8>) -> Self {
        Self { id, len, value }
    }
}

pub fn extract_raw<const LENGTH: usize>(
    reader: &mut impl std::io::Read,
    tags: [u8; LENGTH],
) -> std::io::Result<BTreeMap<u8, RawField>> {
    let mut gathered = BTreeMap::new();

    for _id in tags {
        let id = u8::deserialize(reader)?;
        let len = u16::deserialize(reader)?;

        let mut buf = vec![0; len as usize];

        reader.read_exact(&mut buf)?;

        gathered.insert(id, RawField::new(id, len, buf));
    }

    Ok(gathered)
}
