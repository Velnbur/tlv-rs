pub(crate) mod utils;

pub trait Serialize: Sized {
    fn serialize<W>(&self, writer: &mut W) -> std::io::Result<usize>
    where
        W: std::io::Write;

    fn serialized_length(&self) -> u16 {
        std::mem::size_of::<Self>() as u16
    }
}
