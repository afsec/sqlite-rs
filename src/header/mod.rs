//! Reference: https://www.sqlite.org/fileformat2.html

mod db_filesize_in_pages;
mod file_change_counter;
mod file_format_version_numbers;
mod freelist_pages;
mod magic_header_string;
mod page_size;
mod payload_fractions;
mod reserved_bytes_per_page;
mod schema_cookie;
mod traits;

use core::fmt::Display;

use self::traits::ParseBytes;
pub use self::{
  db_filesize_in_pages::DatabaseFileSizeInPages,
  file_change_counter::FileChangeCounter,
  file_format_version_numbers::{
    FileFormatReadVersion, FileFormatVersionNumbers, FileFormatWriteVersion,
  },
  freelist_pages::FreeListPages,
  magic_header_string::MagicHeaderString,
  page_size::PageSize,
  payload_fractions::{
    LeafPayloadFraction, MaximumEmbeddedPayloadFraction,
    MinimumEmbeddedPayloadFraction, PayloadFractions,
  },
  reserved_bytes_per_page::ReservedBytesPerPage,
  schema_cookie::SchemaCookie,
};
use crate::result::SQLiteError;
use alloc::{borrow::ToOwned, format};

/// # Database File Format
///
/// |Offset | Size  | Description|
/// |-------|-------|------------|
/// |0      | 16    | The header string: "SQLite format 3\000" |
/// |16     | 2     | The database page size in bytes. Must be a power of two between 512 and 32768 inclusive, or the bytes 1 representing a page size of 65536. |
/// |18     | 1     | File format write version. 1 for legacy; 2 for WAL. |
/// |19     | 1     | File format read version. 1 for legacy; 2 for WAL. |
/// |20     | 1     | Bytes of unused "reserved" space at the end of each page. Usually 0. |
/// |21     | 1     | Maximum embedded payload fraction. Must be 64. |
/// |22     | 1     | Minimum embedded payload fraction. Must be 32. |
/// |23     | 1     | Leaf payload fraction. Must be 32. |
/// |24     | 4     | File change counter. |
/// |28     | 4     | Size of the database file in pages. The "in-header database size". |
/// |32     | 4     | Page number of the first freelist trunk page. |
/// |36     | 4     | Total number of freelist pages. |
/// |40     | 4     | The schema cookie. |
/// |44     | 4     | The schema format number. Supported schema formats are 1, 2, 3, and 4. |
/// |48     | 4     | Default page cache size. |
/// |52     | 4     | The page number of the largest root b-tree page when in auto-vacuum or incremental-vacuum modes, or zero otherwise. |
/// |56     | 4     | The database text encoding. A bytes of 1 means UTF-8. A bytes of 2 means UTF-16le. A bytes of 3 means UTF-16be. |
/// |60     | 4     | The "user version" as read and set by the user_version pragma. |
/// |64     | 4     | True (non-zero) for incremental-vacuum mode. False (zero) otherwise. |
/// |68     | 4     | The "Application ID" set by PRAGMA application_id. |
/// |72     | 20    | Reserved for expansion. Must be zero. |
/// |92     | 4     | The version-valid-for number. |
/// |96     | 4     | SQLITE_VERSION_NUMBER |
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
  /// Size of the database file in pages. The "in-header database size".
  db_filesize_in_pages: DatabaseFileSizeInPages,
  /// Unused pages in the database file are stored on a freelist.
  freelist_pages: FreeListPages,
  /// The schema cookie.
  schema_cookie: SchemaCookie,
}

impl SqliteHeader {
  pub fn magic_header_string(&self) -> &MagicHeaderString {
    &self.magic_header_string
  }

  pub fn page_size(&self) -> &PageSize {
    &self.page_size
  }

  pub fn file_format_version_numbers(&self) -> &FileFormatVersionNumbers {
    &self.file_format_version_numbers
  }

  pub fn reserved_bytes_per_page(&self) -> &ReservedBytesPerPage {
    &self.reserved_bytes_per_page
  }

  pub fn payload_fractions(&self) -> &PayloadFractions {
    &self.payload_fractions
  }

  pub fn file_change_counter(&self) -> &FileChangeCounter {
    &self.file_change_counter
  }

  pub fn db_filesize_in_pages(&self) -> &DatabaseFileSizeInPages {
    &self.db_filesize_in_pages
  }

  pub fn freelist_pages(&self) -> &FreeListPages {
    &self.freelist_pages
  }

  pub fn schema_cookie(&self) -> &SchemaCookie {
    &self.schema_cookie
  }
}
impl TryFrom<&[u8; 100]> for SqliteHeader {
  type Error = SQLiteError;

  fn try_from(bytes: &[u8; 100]) -> Result<Self, Self::Error> {
    let bytes = bytes;

    // println!("{:x?}", &bytes[0..=15]);
    // println!("{:x?}", &bytes[16..=17]);
    // println!("{:x?}", &bytes[18..=19]);
    // println!("{:x?}", &bytes[20]);
    // println!("{:x?}", &bytes[21..=23]);
    // println!("{:x?}", &bytes[24..=27]);
    // println!("{:x?}", &bytes[28..=31]);

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

    let freelist_pages = FreeListPages::parse_bytes(&bytes[32..=39])?;

    let schema_cookie = SchemaCookie::parse_bytes(&bytes[40..=43])?;

    Ok(Self {
      magic_header_string,
      page_size,
      file_format_version_numbers,
      reserved_bytes_per_page,
      payload_fractions,
      file_change_counter,
      db_filesize_in_pages,
      freelist_pages,
      schema_cookie,
    })
  }
}

impl Display for SqliteHeader {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    const LABEL_WIDTH: usize = 21;

    let mut output = "".to_owned();
    output.push_str("SQLite Header\n");
    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "database page size:",
      value = **self.page_size()
    ));
    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "write format:",
      value = **self.file_format_version_numbers.write_version()
    ));
    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "read format:",
      value = **self.file_format_version_numbers.read_version()
    ));
    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "reserved bytes:",
      value = **self.reserved_bytes_per_page()
    ));
    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "file change counter:",
      value = **self.file_change_counter()
    ));

    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "database page count:",
      value = **self.db_filesize_in_pages()
    ));

    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "freelist page count:",
      value = **self.freelist_pages().total()
    ));

    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "schema cookie:",
      value = **self.schema_cookie()
    ));

    write!(f, "{output}")
  }
}
/*
$ cat flights.info
database page size:  4096
write format:        1
read format:         1
reserved bytes:      0
file change counter: 4
database page count: 3
freelist page count: 0
schema cookie:       2
schema format:       4
default cache size:  0
autovacuum top root: 0
incremental vacuum:  0
text encoding:       1 (utf8)
user version:        0
application id:      0
software version:    3030000
number of tables:    2
number of indexes:   0
number of triggers:  0
number of views:     0
schema size:         138
data version         1

*/
