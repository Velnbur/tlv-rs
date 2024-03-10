pub use crate::ser::Serialize;

pub fn serialize<T, W>(value: &T, writer: &mut W) -> std::io::Result<usize>
where
    T: Serialize,
    W: std::io::Write,
{
    value.serialize(writer)
}

pub fn serialize_bytes<W>(bytes: &[u8], writer: &mut W) -> std::io::Result<usize>
where
    W: std::io::Write,
{
    let len = bytes.len() as u16;
    len.serialize(writer)?;
    writer.write_all(bytes)?;
    Ok(bytes.len())
}
