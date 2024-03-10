use crate::{Deserialize, Serialize};

impl Serialize for bool {
    fn serialize<W>(&self, writer: &mut W) -> std::io::Result<usize>
    where
        W: std::io::Write,
    {
        (if *self { 1u8 } else { 0u8 }).serialize(writer)
    }
}

impl Serialize for u8 {
    fn serialize<W>(&self, writer: &mut W) -> std::io::Result<usize>
    where
        W: std::io::Write,
    {
        writer.write_all(&[*self])?;
        Ok(1)
    }
}

impl Serialize for u16 {
    fn serialize<W>(&self, writer: &mut W) -> std::io::Result<usize>
    where
        W: std::io::Write,
    {
        writer.write_all(&self.to_le_bytes())?;
        Ok(2)
    }
}

impl Serialize for u32 {
    fn serialize<W>(&self, writer: &mut W) -> std::io::Result<usize>
    where
        W: std::io::Write,
    {
        writer.write_all(&self.to_le_bytes())?;
        Ok(4)
    }
}

impl Serialize for u64 {
    fn serialize<W>(&self, writer: &mut W) -> std::io::Result<usize>
    where
        W: std::io::Write,
    {
        writer.write_all(&self.to_le_bytes())?;
        Ok(8)
    }
}

impl Deserialize for bool {
    fn deserialize<R>(reader: &mut R) -> std::io::Result<Self>
    where
        R: std::io::Read,
    {
        Ok(u8::deserialize(reader)? != 0)
    }
}

impl Deserialize for u8 {
    fn deserialize<R>(reader: &mut R) -> std::io::Result<Self>
    where
        R: std::io::Read,
    {
        let mut buf = [0; 1];
        reader.read_exact(&mut buf)?;
        Ok(buf[0])
    }
}

impl Deserialize for u16 {
    fn deserialize<R>(reader: &mut R) -> std::io::Result<Self>
    where
        R: std::io::Read,
    {
        let mut buf = [0; 2];
        reader.read_exact(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }
}

impl Deserialize for u32 {
    fn deserialize<R>(reader: &mut R) -> std::io::Result<Self>
    where
        R: std::io::Read,
    {
        let mut buf = [0; 4];
        reader.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }
}

impl Deserialize for u64 {
    fn deserialize<R>(reader: &mut R) -> std::io::Result<Self>
    where
        R: std::io::Read,
    {
        let mut buf = [0; 8];
        reader.read_exact(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }
}
