//! Reference: https://www.sqlite.org/fileformat2.html

pub mod file_change_counter;
pub mod file_format_version_numbers;
pub mod magic_header_string;
pub mod page_size;
pub mod payload_fractions;
pub mod reserved_bytes_per_page;

use self::{
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
/// |0	    | 16	| The header string: "SQLite format 3\000" |
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
pub struct SqliteHeader<'a> {
  magic_header_string: MagicHeaderString<'a>,
  page_size: PageSize,
  file_format_version_numbers: FileFormatVersionNumbers,
  reserved_bytes_per_page: ReservedBytesPerPage,
  payload_fractions: PayloadFractions,
  file_change_counter: FileChangeCounter,
}

impl<'a> SqliteHeader<'a> {
  pub fn magic_header_string(&self) -> &MagicHeaderString<'a> {
    &self.magic_header_string
  }

  pub fn page_size(&self) -> &PageSize {
    &self.page_size
  }
}

impl<'a> ParseBytes<&'a [u8; 100]> for SqliteHeader<'a> {
  fn struct_name() -> &'static str {
    "SqliteHeader"
  }

  fn valid_size() -> usize {
    100
  }

  fn parse_bytes(input: &'a [u8; 100]) -> SQLiteResult<Self> {
    let bytes = input;
    Self::check_payload_size(bytes)?;

    println!("{:x?}", &bytes[0..=15]);
    println!("{:x?}", &bytes[16..=17]);
    println!("{:x?}", &bytes[18..=19]);
    println!("{:x?}", &bytes[20]);
    println!("{:x?}", &bytes[21..=23]);
    println!("{:x?}", &bytes[24..=27]);

    let magic_header_string = MagicHeaderString::parse_bytes(&bytes[0..=15])?;
    let page_size = PageSize::parse_bytes(&bytes[16..=17])?;
    let file_format_version_numbers =
      FileFormatVersionNumbers::parse_bytes(&bytes[18..=19])?;
    let reserved_bytes_per_page =
      ReservedBytesPerPage::parse_bytes((&page_size, bytes[20]))?;
    let payload_fractions = PayloadFractions::parse_bytes(&bytes[21..=23])?;

    let file_change_counter = FileChangeCounter::parse_bytes(&bytes[24..=27])?;
    Ok(Self {
      magic_header_string,
      page_size,
      file_format_version_numbers,
      reserved_bytes_per_page,
      payload_fractions,
      file_change_counter,
    })
  }
}

trait ParseBytes<T>
where
  Self: Sized,
{
  fn struct_name() -> &'static str;
  fn valid_size() -> usize;
  fn check_payload_size(bytes: &[u8]) -> SQLiteResult<()> {
    if bytes.len() != Self::valid_size() {
      bail!("Invalid size for {}", Self::struct_name());
    } else {
      Ok(())
    }
  }
  fn parse_bytes(input: T) -> SQLiteResult<Self>;
}
