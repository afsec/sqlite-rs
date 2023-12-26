use core::ops::Deref;

use crate::traits::{Name, ParseBytes};
use crate::{
  field_parsing_error, impl_name,
  result::{SqliteError, SqliteResult},
};

/// # Payload Fractions (3 Bytes)
///
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

impl_name! {PayloadFractions}

impl ParseBytes for PayloadFractions {
  const LENGTH_BYTES: usize = 3;

  fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self> {
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

impl Deref for MaximumEmbeddedPayloadFraction {
  type Target = u8;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl_name! {MaximumEmbeddedPayloadFraction}

impl ParseBytes for MaximumEmbeddedPayloadFraction {
  const LENGTH_BYTES: usize = 1;

  fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self> {
    let maximum = *bytes
      .first()
      .ok_or(field_parsing_error! {Self::NAME.into()})?;
    if maximum == 64 {
      Ok(Self(maximum))
    } else {
      Err(SqliteError::Custom(
        "MaximumEmbeddedPayloadFraction must be 64.".into(),
      ))
    }
  }
}

/// Minimum embedded payload fraction. Must be 32.
#[derive(Debug)]
pub struct MinimumEmbeddedPayloadFraction(u8);

impl Deref for MinimumEmbeddedPayloadFraction {
  type Target = u8;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl_name! {MinimumEmbeddedPayloadFraction}

impl ParseBytes for MinimumEmbeddedPayloadFraction {
  const LENGTH_BYTES: usize = 1;

  fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self> {
    let minimum = *bytes
      .first()
      .ok_or(field_parsing_error! {Self::NAME.into()})?;
    if minimum == 32 {
      Ok(Self(minimum))
    } else {
      Err(SqliteError::Custom(
        "MinimumEmbeddedPayloadFraction must be 32.".into(),
      ))
    }
  }
}

/// Leaf payload fraction. Must be 32.
#[derive(Debug)]
pub struct LeafPayloadFraction(u8);

impl Deref for LeafPayloadFraction {
  type Target = u8;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl_name! {LeafPayloadFraction}

impl ParseBytes for LeafPayloadFraction {
  const LENGTH_BYTES: usize = 1;

  fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self> {
    let leaf = *bytes
      .first()
      .ok_or(field_parsing_error! {Self::NAME.into()})?;
    if leaf == 32 {
      Ok(Self(leaf))
    } else {
      Err(SqliteError::Custom(
        "LeafPayloadFraction must be 32.".into(),
      ))
    }
  }
}
