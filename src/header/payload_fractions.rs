use super::traits::ParseBytes;
use crate::result::SQLiteError;
use alloc::format;

/// # Payload Fractions (3 Bytes)
///  The maximum and minimum embedded payload fractions and the leaf payload
/// fraction values must be 64, 32, and 32. These values were originally
/// intended to be tunable parameters that could be used to modify the storage
/// format of the b-tree algorithm. However, that functionality is not
/// supported and there are no current plans to add support in the future.
/// Hence, these three bytes are fixed at the values specified.
#[derive(Debug)]
pub struct PayloadFractions {
  /// Maximum embedded payload fraction. Must be 64.
  maximum: MaximumEmbeddedPayloadFraction,
  /// Minimum embedded payload fraction. Must be 32.
  minimum: MinimumEmbeddedPayloadFraction,
  /// Leaf payload fraction. Must be 32.
  leaf: LeafPayloadFraction,
}

impl PayloadFractions {
  pub fn maximum(&self) -> &MaximumEmbeddedPayloadFraction {
    &self.maximum
  }

  pub fn minimum(&self) -> &MinimumEmbeddedPayloadFraction {
    &self.minimum
  }

  pub fn leaf(&self) -> &LeafPayloadFraction {
    &self.leaf
  }
}

impl ParseBytes<&[u8]> for PayloadFractions {
  fn struct_name() -> &'static str {
    "PayloadFractions"
  }

  fn bytes_length() -> usize {
    3
  }

  fn parsing_handler(bytes: &[u8]) -> crate::result::SQLiteResult<Self> {
    let maximum = MaximumEmbeddedPayloadFraction::parse_bytes(&[bytes[0]])?;
    let minimum = MinimumEmbeddedPayloadFraction::parse_bytes(&[bytes[1]])?;
    let leaf = LeafPayloadFraction::parse_bytes(&[bytes[2]])?;
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

impl ParseBytes<&[u8]> for MaximumEmbeddedPayloadFraction {
  fn struct_name() -> &'static str {
    "MaximumEmbeddedPayloadFraction"
  }

  fn bytes_length() -> usize {
    1
  }

  fn parsing_handler(bytes: &[u8]) -> crate::result::SQLiteResult<Self> {
    let maximum = *bytes.first().ok_or(SQLiteError::Custom(format!(
      "Impossible state on parsing {}",
      Self::struct_name()
    )))?;
    if maximum == 64 {
      Ok(Self(maximum))
    } else {
      Err(SQLiteError::msg(
        "Maximum embedded payload fraction. Must be 64.",
      ))
    }
  }
}

/// Minimum embedded payload fraction. Must be 32.
#[derive(Debug)]
pub struct MinimumEmbeddedPayloadFraction(u8);

impl ParseBytes<&[u8]> for MinimumEmbeddedPayloadFraction {
  fn struct_name() -> &'static str {
    "MinimumEmbeddedPayloadFraction"
  }

  fn bytes_length() -> usize {
    1
  }

  fn parsing_handler(bytes: &[u8]) -> crate::result::SQLiteResult<Self> {
    let minimum = *bytes.first().ok_or(SQLiteError::Custom(format!(
      "Impossible state on parsing {}",
      Self::struct_name()
    )))?;
    if minimum == 32 {
      Ok(Self(minimum))
    } else {
      Err(SQLiteError::msg(
        "Minimum embedded payload fraction. Must be 32.",
      ))
    }
  }
}

/// Leaf payload fraction. Must be 32.
#[derive(Debug)]
pub struct LeafPayloadFraction(u8);

impl ParseBytes<&[u8]> for LeafPayloadFraction {
  fn struct_name() -> &'static str {
    "LeafPayloadFraction"
  }

  fn bytes_length() -> usize {
    1
  }

  fn parsing_handler(bytes: &[u8]) -> crate::result::SQLiteResult<Self> {
    let leaf = *bytes.first().ok_or(SQLiteError::Custom(format!(
      "Impossible state on parsing {}",
      Self::struct_name()
    )))?;
    if leaf == 32 {
      Ok(Self(leaf))
    } else {
      Err(SQLiteError::msg("Leaf payload fraction. Must be 32."))
    }
  }
}
