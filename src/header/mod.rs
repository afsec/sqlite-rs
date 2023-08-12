//! Reference: https://www.sqlite.org/fileformat2.html

pub(self) mod db_filesize_in_pages;
pub(self) mod file_change_counter;
pub(self) mod file_format_version_numbers;
pub(self) mod magic_header_string;
pub(self) mod page_size;
pub(self) mod payload_fractions;
pub(self) mod reserved_bytes_per_page;

use self::{
  db_filesize_in_pages::DatabaseFileSizeInPages,
  file_change_counter::FileChangeCounter,
  file_format_version_numbers::FileFormatVersionNumbers,
  magic_header_string::MagicHeaderString, page_size::PageSize,
};
use crate::{
  header::{
    payload_fractions::PayloadFractions,
    reserved_bytes_per_page::ReservedBytesPerPage,
  },
  result::{SQLiteError, SQLiteResult},
};
use anyhow::bail;

/// # Database File Format
///
/// |Offset	| Size	| Description|
/// |-------|-------|------------|
/// |0	    | 16	  | The header string: "SQLite format 3\000" |
/// |16	    | 2	    | The database page size in bytes. Must be a power of two between 512 and 32768 inclusive, or the bytes 1 representing a page size of 65536. |
/// |18	    | 1	    | File format write version. 1 for legacy; 2 for WAL. |
/// |19	    | 1	    | File format read version. 1 for legacy; 2 for WAL. |
/// |20	    | 1	    | Bytes of unused "reserved" space at the end of each page. Usually 0. |
/// |21	    | 1	    | Maximum embedded payload fraction. Must be 64. |
/// |22	    | 1	    | Minimum embedded payload fraction. Must be 32. |
/// |23	    | 1	    | Leaf payload fraction. Must be 32. |
/// |24	    | 4	    | File change counter. |
/// |28	    | 4	    | Size of the database file in pages. The "in-header database size". |
/// |32	    | 4	    | Page number of the first freelist trunk page. |
/// |36	    | 4	    | Total number of freelist pages. |
/// |40	    | 4	    | The schema cookie. |
/// |44	    | 4	    | The schema format number. Supported schema formats are 1, 2, 3, and 4. |
/// |48	    | 4	    | Default page cache size. |
/// |52	    | 4	    | The page number of the largest root b-tree page when in auto-vacuum or incremental-vacuum modes, or zero otherwise. |
/// |56	    | 4	    | The database text encoding. A bytes of 1 means UTF-8. A bytes of 2 means UTF-16le. A bytes of 3 means UTF-16be. |
/// |60	    | 4	    | The "user version" as read and set by the user_version pragma. |
/// |64	    | 4	    | True (non-zero) for incremental-vacuum mode. False (zero) otherwise. |
/// |68	    | 4	    | The "Application ID" set by PRAGMA application_id. |
/// |72	    | 20	| Reserved for expansion. Must be zero. |
/// |92	    | 4	    | The version-valid-for number. |
/// |96	    | 4	    | SQLITE_VERSION_NUMBER |
#[derive(Debug)]
pub struct SqliteHeader {
  /// The header string: "`SQLite format 3\000`".
  magic_header_string: MagicHeaderString,
  /// The database page size in bytes.
  ///  Must be a power of two between 512 and 32768 inclusive,
  /// or the bytes 1 representing a page size of 65536.
  page_size: PageSize,
  /// File format version numbers.
  file_format_version_numbers: FileFormatVersionNumbers,
  /// Bytes of unused "reserved" space at the end of each page. Usually 0.
  reserved_bytes_per_page: ReservedBytesPerPage,
  /// Payload Fractions.
  payload_fractions: PayloadFractions,
  /// File change counter.
  file_change_counter: FileChangeCounter,
  // Size of the database file in pages. The "in-header database size".
  db_filesize_in_pages: DatabaseFileSizeInPages,
}

impl SqliteHeader {
  pub fn magic_header_string(&self) -> &MagicHeaderString {
    &self.magic_header_string
  }

  pub fn page_size(&self) -> &PageSize {
    &self.page_size
  }
}
impl TryFrom<&[u8; 100]> for SqliteHeader {
  type Error = SQLiteError;

  fn try_from(bytes: &[u8; 100]) -> Result<Self, Self::Error> {
    let bytes = bytes;

    println!("{:x?}", &bytes[0..=15]);
    println!("{:x?}", &bytes[16..=17]);
    println!("{:x?}", &bytes[18..=19]);
    println!("{:x?}", &bytes[20]);
    println!("{:x?}", &bytes[21..=23]);
    println!("{:x?}", &bytes[24..=27]);
    println!("{:x?}", &bytes[28..=31]);

    let magic_header_string = MagicHeaderString::parse_bytes(&bytes[0..=15])?;
    let page_size = PageSize::parse_bytes(&bytes[16..=17])?;
    let file_format_version_numbers =
      FileFormatVersionNumbers::parse_bytes(&bytes[18..=19])?;
    let reserved_bytes_per_page =
      ReservedBytesPerPage::parse_bytes(&[bytes[20]])?;
    let payload_fractions = PayloadFractions::parse_bytes(&bytes[21..=23])?;

    let file_change_counter = FileChangeCounter::parse_bytes(&bytes[24..=27])?;
    let db_filesize_in_pages =
      DatabaseFileSizeInPages::parse_bytes(&bytes[28..=31])?;
    Ok(Self {
      magic_header_string,
      page_size,
      file_format_version_numbers,
      reserved_bytes_per_page,
      payload_fractions,
      file_change_counter,
      db_filesize_in_pages,
    })
  }
}

trait ParseBytes<T>
where
  Self: Sized,
{
  fn struct_name() -> &'static str;
  fn bytes_length() -> usize;
  fn parsing_handler(bytes: &[u8]) -> SQLiteResult<Self>;

  fn check_payload_size(bytes: &[u8]) -> SQLiteResult<()> {
    if bytes.len() != Self::bytes_length() {
      bail!("Invalid size for {}", Self::struct_name());
    } else {
      Ok(())
    }
  }
  fn parse_bytes(bytes: &[u8]) -> SQLiteResult<Self> {
    Self::check_payload_size(bytes)?;
    Self::parsing_handler(bytes)
  }
}

// TODO
trait ValidateParsed<T>
where
  Self: Sized + ParseBytes<T>,
{
  fn validate_parsed(&self) -> SQLiteResult<()>;
}
