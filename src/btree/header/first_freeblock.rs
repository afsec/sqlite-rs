use crate::{impl_name, result::SQLiteResult, traits::ParseBytes};

/// # BtreePageFirstFreeBlock (2 Bytes)
///  A freeblock is a structure used to identify unallocated space within a
/// b-tree page. Freeblocks are organized as a chain. The first 2 bytes of a
/// freeblock are a big-endian integer which is the offset in the b-tree page of
/// the next freeblock in the chain, or zero if the freeblock is the last on the
/// chain. The third and fourth bytes of each freeblock form a big-endian
/// integer which is the size of the freeblock in bytes, including the 4-byte
/// header. Freeblocks are always connected in order of increasing offset. The
/// second field of the b-tree page header is the offset of the first freeblock,
/// or zero if there are no freeblocks on the page. In a well-formed b-tree
/// page, there will always be at least one cell before the first freeblock.
#[derive(Debug, PartialEq, Eq)]
pub struct BtreePageFirstFreeBlock(u16);

impl_name! {BtreePageFirstFreeBlock}

impl ParseBytes for BtreePageFirstFreeBlock {
  const LENGTH_BYTES: usize = 2;

  fn parsing_handler(bytes: &[u8]) -> SQLiteResult<Self> {
    let value = u16::from_be_bytes([bytes[0], bytes[1]]);
    Ok(Self(value))
  }
}
