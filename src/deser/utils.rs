use crate::deser::Deserialize;

pub fn deserialize<T, R>(reader: &mut R) -> std::io::Result<T>
where
    T: Deserialize,
    R: std::io::Read,
{
    T::deserialize(reader)
}

pub fn deserialize_bytes<R>(reader: &mut R) -> std::io::Result<Vec<u8>>
where
    R: std::io::Read,
{
    let len = u32::deserialize(reader)? as usize;
    let mut buf = vec![0; len];
    reader.read_exact(&mut buf)?;
    Ok(buf)
}
