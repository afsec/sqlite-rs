use anyhow::bail;

use crate::result::SQLiteError;

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

impl<'a> TryFrom<&'a [u8]> for PayloadFractions {
    type Error = SQLiteError;

    fn try_from(payload: &'a [u8]) -> Result<Self, Self::Error> {
        const VALID_SIZE: usize = 3;

        if payload.len() != VALID_SIZE {
            bail!("Invalid size for PayloadFractions");
        }
        let maximum = MaximumEmbeddedPayloadFraction::try_from(payload[0])?;
        let minimum = MinimumEmbeddedPayloadFraction::try_from(payload[1])?;
        let leaf = LeafPayloadFraction::try_from(payload[2])?;
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

impl TryFrom<u8> for MaximumEmbeddedPayloadFraction {
    type Error = SQLiteError;

    fn try_from(maximum: u8) -> Result<Self, Self::Error> {
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
impl TryFrom<u8> for MinimumEmbeddedPayloadFraction {
    type Error = SQLiteError;

    fn try_from(minimum: u8) -> Result<Self, Self::Error> {
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
impl TryFrom<u8> for LeafPayloadFraction {
    type Error = SQLiteError;

    fn try_from(leaf: u8) -> Result<Self, Self::Error> {
        if leaf == 32 {
            Ok(Self(leaf))
        } else {
            bail!("Leaf payload fraction. Must be 32.")
        }
    }
}
