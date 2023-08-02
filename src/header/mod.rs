//! Reference: https://www.sqlite.org/fileformat2.html

mod file_format_version_numbers;
mod magic_header_string;
mod page_size;
mod payload_fractions;
mod reserved_bytes_per_page;

use self::{
    file_format_version_numbers::FileFormatVersionNumbers, magic_header_string::MagicHeaderString,
    page_size::PageSize,
};
use crate::{
    header::{payload_fractions::PayloadFractions, reserved_bytes_per_page::ReservedBytesPerPage},
    helpers::SQLiteError,
};

/// # Database File Format
///
/// |Offset	| Size	| Description|
/// |-------|-------|------------|
/// |0	    | 16	| The header string: "SQLite format 3\000" |
/// |16	    | 2	    | The database page size in bytes. Must be a power of two between 512 and 32768 inclusive, or the value 1 representing a page size of 65536. |
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
/// |56	    | 4	    | The database text encoding. A value of 1 means UTF-8. A value of 2 means UTF-16le. A value of 3 means UTF-16be. |
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
}

impl<'a> SqliteHeader<'a> {
    pub fn magic_header_string(&self) -> &MagicHeaderString<'a> {
        &self.magic_header_string
    }

    pub fn page_size(&self) -> &PageSize {
        &self.page_size
    }
}

impl<'a> TryFrom<&'a [u8; 100]> for SqliteHeader<'a> {
    type Error = SQLiteError;

    fn try_from(value: &'a [u8; 100]) -> Result<Self, Self::Error> {
        println!("{:x?}", &value[0..=15]);
        println!("{:x?}", &value[16..=17]);
        println!("{:x?}", &value[18..=19]);
        println!("{:x?}", &value[20]);
        println!("{:x?}", &value[21..=23]);

        let magic_header_string = MagicHeaderString::try_from(&value[0..=15])?;
        let page_size = PageSize::try_from(&value[16..=17])?;
        let file_format_version_numbers = FileFormatVersionNumbers::try_from(&value[18..=19])?;
        let reserved_bytes_per_page = ReservedBytesPerPage::try_from((&page_size, value[20]))?;
        let payload_fractions = PayloadFractions::try_from(&value[21..=23])?;
        Ok(Self {
            magic_header_string,
            page_size,
            file_format_version_numbers,
            reserved_bytes_per_page,
            payload_fractions,
        })
    }
}
