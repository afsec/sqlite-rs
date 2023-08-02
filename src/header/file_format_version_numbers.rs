use crate::helpers::SQLiteError;
use anyhow::bail;

/// # File format version numbers (2 Bytes)
///  The file format write version and file format read version at offsets 18
/// and 19 are intended to allow for enhancements of the file format in future
/// versions of SQLite. In current versions of SQLite, both of these values
/// are:
///   - `1` for rollback journalling modes; and
///   - `2` for WAL journalling mode.
///
///  If a version of SQLite coded to the current file format specification
/// encounters a database file where the read version is 1 or 2 but the write
/// version is greater than 2, then the database file must be treated as
/// read-only. If a database file with a read version greater than 2 is
/// encountered, then that database cannot be read or written.
#[derive(Debug)]
pub struct FileFormatVersionNumbers {
    write_version: FileFormatWriteVersion,
    read_version: FileFormatReadVersion,
}
impl<'a> TryFrom<&'a [u8]> for FileFormatVersionNumbers {
    type Error = SQLiteError;

    fn try_from(payload: &'a [u8]) -> Result<Self, Self::Error> {
        if payload.len() != 2 {
            bail!("Invalid size for FileFormatVersionNumbers")
        }
        let write_version = FileFormatWriteVersion::try_from(payload[0])?;
        let read_version = FileFormatReadVersion::try_from(payload[1])?;
        Ok(Self {
            write_version,
            read_version,
        })
    }
}
#[derive(Debug)]
pub enum FileFormatWriteVersion {
    Legacy,
    WAL,
}

impl TryFrom<u8> for FileFormatWriteVersion {
    type Error = SQLiteError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Legacy),
            2 => Ok(Self::WAL),
            _ => bail!("Invalid payload for FileFormatWriteVersion"),
        }
    }
}

#[derive(Debug)]
pub enum FileFormatReadVersion {
    Legacy,
    WAL,
}
impl TryFrom<u8> for FileFormatReadVersion {
    type Error = SQLiteError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Legacy),
            2 => Ok(Self::WAL),
            _ => bail!("Invalid payload for FileFormatReadVersion"),
        }
    }
}
