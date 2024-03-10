use crate::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

pub mod num;

impl<const LENGTH: usize> Serialize for [u8; LENGTH] {
    fn serialize<W>(&self, writer: &mut W) -> std::io::Result<usize>
    where
        W: std::io::Write,
    {
        writer.write_all(self)?;
        Ok(LENGTH)
    }

    fn serialized_length(&self) -> u16 {
        LENGTH as u16
    }
}

impl<T> Serialize for Option<T>
where
    T: Serialize,
{
    fn serialize<W>(&self, writer: &mut W) -> std::io::Result<usize>
    where
        W: std::io::Write,
    {
        match self {
            Some(value) => {
                let mut len = 0;
                len += 1u8.serialize(writer)?;
                len += value.serialize(writer)?;
                Ok(len)
            }
            None => {
                0u8.serialize(writer)?;
                Ok(1)
            }
        }
    }

    fn serialized_length(&self) -> u16 {
        match self {
            Some(value) => 1 + value.serialized_length(),
            None => 1,
        }
    }
}

impl<T> Serialize for Vec<T>
where
    T: Serialize,
{
    fn serialize<W>(&self, writer: &mut W) -> std::io::Result<usize>
    where
        W: std::io::Write,
    {
        let mut len = 0;
        len += (self.len() as u32).serialize(writer)?;
        for item in self {
            len += item.serialize(writer)?;
        }
        Ok(len)
    }

    fn serialized_length(&self) -> u16 {
        let mut len = std::mem::size_of::<u32>() as u16;
        for item in self {
            len += item.serialized_length();
        }
        len
    }
}

impl<K, V> Serialize for HashMap<K, V>
where
    K: Serialize,
    V: Serialize,
{
    fn serialize<W>(&self, writer: &mut W) -> std::io::Result<usize>
    where
        W: std::io::Write,
    {
        let mut len = 0;
        len += (self.len() as u32).serialize(writer)?;
        for (key, value) in self {
            len += key.serialize(writer)?;
            len += value.serialize(writer)?;
        }
        Ok(len)
    }

    fn serialized_length(&self) -> u16 {
        let mut len = std::mem::size_of::<u32>() as u16;
        for (key, value) in self {
            len += key.serialized_length();
            len += value.serialized_length();
        }
        len
    }
}

impl<K, V> Serialize for BTreeMap<K, V>
where
    K: Serialize,
    V: Serialize,
{
    fn serialize<W>(&self, writer: &mut W) -> std::io::Result<usize>
    where
        W: std::io::Write,
    {
        let mut len = 0;
        len += (self.len() as u32).serialize(writer)?;
        for (key, value) in self {
            len += key.serialize(writer)?;
            len += value.serialize(writer)?;
        }
        Ok(len)
    }

    fn serialized_length(&self) -> u16 {
        let mut len = std::mem::size_of::<u32>() as u16;
        for (key, value) in self {
            len += key.serialized_length();
            len += value.serialized_length();
        }
        len
    }
}

impl<const LENGTH: usize> Deserialize for [u8; LENGTH] {
    fn deserialize<R>(reader: &mut R) -> std::io::Result<Self>
    where
        R: std::io::Read,
    {
        let mut buf = [0; LENGTH];
        reader.read_exact(&mut buf)?;
        Ok(buf)
    }
}

impl<T> Deserialize for Option<T>
where
    T: Deserialize,
{
    fn deserialize<R>(reader: &mut R) -> std::io::Result<Self>
    where
        R: std::io::Read,
    {
        let has_value = u8::deserialize(reader)?;
        if has_value == 0 {
            Ok(None)
        } else {
            Ok(Some(T::deserialize(reader)?))
        }
    }
}

impl<T> Deserialize for Vec<T>
where
    T: Deserialize,
{
    fn deserialize<R>(reader: &mut R) -> std::io::Result<Self>
    where
        R: std::io::Read,
    {
        let len = u32::deserialize(reader)? as usize;
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(T::deserialize(reader)?);
        }
        Ok(vec)
    }
}

impl<K, V> Deserialize for HashMap<K, V>
where
    K: Deserialize + Eq + std::hash::Hash,
    V: Deserialize,
{
    fn deserialize<R>(reader: &mut R) -> std::io::Result<Self>
    where
        R: std::io::Read,
    {
        let len = u32::deserialize(reader)? as usize;
        let mut map = HashMap::with_capacity(len);
        for _ in 0..len {
            let key = K::deserialize(reader)?;
            let value = V::deserialize(reader)?;
            map.insert(key, value);
        }
        Ok(map)
    }
}

impl<K, V> Deserialize for BTreeMap<K, V>
where
    K: Deserialize + Ord,
    V: Deserialize,
{
    fn deserialize<R>(reader: &mut R) -> std::io::Result<Self>
    where
        R: std::io::Read,
    {
        let len = u32::deserialize(reader)? as usize;
        let mut map = BTreeMap::new();
        for _ in 0..len {
            let key = K::deserialize(reader)?;
            let value = V::deserialize(reader)?;
            map.insert(key, value);
        }
        Ok(map)
    }
}

impl Serialize for String {
    fn serialize<W>(&self, writer: &mut W) -> std::io::Result<usize>
    where
        W: std::io::Write,
    {
        let bytes = self.as_bytes();
        let len = bytes.len() as u32;
        len.serialize(writer)?;
        writer.write_all(bytes)?;
        Ok(len as usize + bytes.len())
    }

    fn serialized_length(&self) -> u16 {
        let bytes = self.as_bytes();
        (std::mem::size_of::<u16>() + bytes.len()) as u16
    }
}

impl Deserialize for String {
    fn deserialize<R>(reader: &mut R) -> std::io::Result<Self>
    where
        R: std::io::Read,
    {
        let len = u32::deserialize(reader)? as usize;
        let mut buf = vec![0; len];
        reader.read_exact(&mut buf)?;
        String::from_utf8(buf).map_err(|_| std::io::ErrorKind::InvalidData.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u8() {
        let mut buf = Vec::new();
        42u8.serialize(&mut buf).unwrap();
        assert_eq!(buf, vec![42]);
        assert_eq!(u8::deserialize(&mut buf.as_slice()).unwrap(), 42);
    }

    #[test]
    fn test_u16() {
        let mut buf = Vec::new();
        42u16.serialize(&mut buf).unwrap();
        assert_eq!(buf, vec![42, 0]);
        assert_eq!(u16::deserialize(&mut buf.as_slice()).unwrap(), 42);
    }

    #[test]
    fn test_u32() {
        let mut buf = Vec::new();
        42u32.serialize(&mut buf).unwrap();
        assert_eq!(buf, vec![42, 0, 0, 0]);
        assert_eq!(u32::deserialize(&mut buf.as_slice()).unwrap(), 42);
    }

    #[test]
    fn test_u64() {
        let mut buf = Vec::new();
        42u64.serialize(&mut buf).unwrap();
        assert_eq!(buf, vec![42, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(u64::deserialize(&mut buf.as_slice()).unwrap(), 42);
    }

    #[test]
    fn test_vec() {
        let mut buf = Vec::new();
        vec![1u8, 2, 3].serialize(&mut buf).unwrap();
        assert_eq!(buf, vec![3, 0, 0, 0, 1, 2, 3]);
        //                   ^- length is 4 bytes
        assert_eq!(
            Vec::<u8>::deserialize(&mut buf.as_slice()).unwrap(),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn test_hash_map() {
        let mut buf = Vec::new();
        let mut map = HashMap::new();
        map.insert(1u8, 2u8);
        map.insert(3, 4);
        map.serialize(&mut buf).unwrap();
        assert_eq!(buf, vec![2, 0, 0, 0, 1, 2, 3, 4]);
        //                   ^- length is 4 bytes
        assert_eq!(
            HashMap::<u8, u8>::deserialize(&mut buf.as_slice()).unwrap(),
            map
        );
    }

    #[test]
    fn test_btree_map() {
        let mut buf = Vec::new();
        let mut map = BTreeMap::new();
        map.insert(1u8, 2u8);
        map.insert(3, 4);
        map.serialize(&mut buf).unwrap();
        assert_eq!(buf, vec![2, 0, 0, 0, 1, 2, 3, 4]);
        //                   ^- length is 4 bytes
        assert_eq!(
            BTreeMap::<u8, u8>::deserialize(&mut buf.as_slice()).unwrap(),
            map
        );
    }

    #[test]
    fn test_serialized_length() {
        assert_eq!(42u8.serialized_length(), 1);
        assert_eq!(42u16.serialized_length(), 2);
        assert_eq!(42u32.serialized_length(), 4);
        assert_eq!(42u64.serialized_length(), 8);
        assert_eq!(
            vec![1u8, 2, 3].serialized_length(),
            4 + 3,
            "length (4 bytes) + 3 bytes"
        );
        let mut map = HashMap::new();
        map.insert(1u8, 2u8);
        map.insert(3, 4);
        assert_eq!(map.serialized_length(), 8);
        let mut map = BTreeMap::new();
        map.insert(1u8, 2u8);
        map.insert(3, 4);
        assert_eq!(map.serialized_length(), 8);
    }
}
