use super::ParseBytes;
use anyhow::bail;

///  The maximum and minimum embedded payload fractions and the leaf payload
/// fraction values must be 64, 32, and 32. These values were originally
/// intended to be tunable parameters that could be used to modify the storage
/// format of the b-tree algorithm. However, that functionality is not
/// supported and there are no current plans to add support in the future.
/// Hence, these three bytes are fixed at the values specified.
#[derive(Debug)]
pub struct PayloadFractions {
  maximum: MaximumEmbeddedPayloadFraction,
  minimum: MinimumEmbeddedPayloadFraction,
  leaf: LeafPayloadFraction,
}

impl ParseBytes<&[u8]> for PayloadFractions {
  fn struct_name() -> &'static str {
    "PayloadFractions"
  }

  fn valid_size() -> usize {
    3
  }

  fn parse_bytes(input: &[u8]) -> crate::result::SQLiteResult<Self> {
    let bytes = input;
    Self::check_payload_size(bytes)?;
    let maximum = MaximumEmbeddedPayloadFraction::parse_bytes(bytes[0])?;
    let minimum = MinimumEmbeddedPayloadFraction::parse_bytes(bytes[1])?;
    let leaf = LeafPayloadFraction::parse_bytes(bytes[2])?;
    Ok(Self {
      maximum,
      minimum,
      leaf,
    })
  }
}

/// Maximum embedded payload fraction. Must be 64.
#[derive(Debug)]
pub struct MaximumEmbeddedPayloadFraction(u8);

impl ParseBytes<u8> for MaximumEmbeddedPayloadFraction {
  fn struct_name() -> &'static str {
    "MaximumEmbeddedPayloadFraction"
  }

  fn valid_size() -> usize {
    1
  }

  fn parse_bytes(maximum: u8) -> crate::result::SQLiteResult<Self> {
    if maximum == 64 {
      Ok(Self(maximum))
    } else {
      bail!("Maximum embedded payload fraction. Must be 64.")
    }
  }
}

/// Minimum embedded payload fraction. Must be 32.
#[derive(Debug)]
pub struct MinimumEmbeddedPayloadFraction(u8);
impl ParseBytes<u8> for MinimumEmbeddedPayloadFraction {
  fn struct_name() -> &'static str {
    "MinimumEmbeddedPayloadFraction"
  }

  fn valid_size() -> usize {
    1
  }

  fn parse_bytes(minimum: u8) -> crate::result::SQLiteResult<Self> {
    if minimum == 32 {
      Ok(Self(minimum))
    } else {
      bail!("Minimum embedded payload fraction. Must be 32.")
    }
  }
}

/// Leaf payload fraction. Must be 32.
#[derive(Debug)]
pub struct LeafPayloadFraction(u8);
impl ParseBytes<u8> for LeafPayloadFraction {
  fn struct_name() -> &'static str {
    "LeafPayloadFraction"
  }

  fn valid_size() -> usize {
    1
  }

  fn parse_bytes(leaf: u8) -> crate::result::SQLiteResult<Self> {
    if leaf == 32 {
      Ok(Self(leaf))
    } else {
      bail!("Leaf payload fraction. Must be 32.")
    }
  }
}
