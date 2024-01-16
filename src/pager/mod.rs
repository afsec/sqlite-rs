pub mod page;

use std::num::NonZeroU32;

use crate::{
  header::{
    FileFormatReadVersion, FileFormatWriteVersion, MagicHeaderString, PageSize,
    ReservedBytesPerPage,
  },
  io::SqliteIo,
  result::{SqliteError, SqliteResult},
  traits::ParseBytes,
};

use self::page::Page;

#[derive(Debug)]
pub struct SqlitePager {
  io: SqliteIo,
  page_size: PageSize,
  reserved_bytes_per_page: ReservedBytesPerPage,
  // cur_page_number: usize,
  // btree_page_header: BtreePageHeader,
}

impl SqlitePager {
  pub fn connect(mut io: SqliteIo) -> SqliteResult<Self> {
    io.rewind()?;
    const BYTES_TO_READ: usize = MagicHeaderString::LENGTH_BYTES
      + PageSize::LENGTH_BYTES
      + FileFormatWriteVersion::LENGTH_BYTES
      + FileFormatReadVersion::LENGTH_BYTES
      + ReservedBytesPerPage::LENGTH_BYTES;
    let mut buf = [0u8; BYTES_TO_READ];

    io.read(&mut buf)?;

    Ok(Self {
      io,
      page_size: PageSize::parse_bytes(&buf[16..=17])?,
      reserved_bytes_per_page: ReservedBytesPerPage::parse_bytes(&[buf[20]])?,
      // cur_page_pos: 1,
    })
  }
  pub fn first(&mut self) -> SqliteResult<Page> {
    self.read(1)
  }

  pub fn read(&mut self, page_number: u32) -> SqliteResult<Page> {
    let page_number = NonZeroU32::new(page_number)
      .ok_or(SqliteError::Custom("page number can't be zero `0`.".into()))?
      .get();
    let page_size = self.page_size().clone();
    let offset_from_start = (page_number - 1) * u32::from(&page_size);
    // dbg!(&offset_from_start);
    self.io.seek(offset_from_start.into())?;

    match page_size {
      PageSize::L512 => {
        const BUF_SIZE: usize = 512;
        let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
        // TODO: Write tests
        self.io.read(&mut buf)?;

        Ok(Page {
          length: page_size,
          raw_data: buf.to_vec(),
        })
      }
      PageSize::L1024 => {
        const BUF_SIZE: usize = 1024;
        let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
        self.io.read(&mut buf)?;

        Ok(Page {
          length: page_size,
          raw_data: buf.to_vec(),
        })
      }
      PageSize::L2048 => {
        const BUF_SIZE: usize = 2048;
        let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
        self.io.read(&mut buf)?;

        Ok(Page {
          length: page_size,
          raw_data: buf.to_vec(),
        })
      }
      PageSize::L4096 => {
        const BUF_SIZE: usize = 4096;
        let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
        self.io.read(&mut buf)?;

        Ok(Page {
          length: page_size,
          raw_data: buf.to_vec(),
        })
      }
      PageSize::L8192 => {
        const BUF_SIZE: usize = 8192;
        let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
        self.io.read(&mut buf)?;

        Ok(Page {
          length: page_size,
          raw_data: buf.to_vec(),
        })
      }
      PageSize::L16384 => {
        const BUF_SIZE: usize = 16384;
        let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
        self.io.read(&mut buf)?;

        Ok(Page {
          length: page_size,
          raw_data: buf.to_vec(),
        })
      }
      PageSize::L32768 => {
        const BUF_SIZE: usize = 32768;
        let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
        self.io.read(&mut buf)?;
        let mut final_buffer = [0u8; Page::MAX_LENGTH];
        for (idx, byte) in buf.iter().enumerate() {
          final_buffer[idx] = *byte;
        }
        Ok(Page {
          length: page_size,
          raw_data: buf.to_vec(),
        })
      }
      PageSize::L65536 => {
        const BUF_SIZE: usize = 65536;
        let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
        self.io.read(&mut buf)?;
        let mut final_buffer = [0u8; Page::MAX_LENGTH];
        for (idx, byte) in buf.iter().enumerate() {
          final_buffer[idx] = *byte;
        }
        Ok(Page {
          length: page_size,
          raw_data: buf.to_vec(),
        })
      }
    }
  }

  pub fn page_size(&self) -> &PageSize {
    &self.page_size
  }

  pub fn reserved_bytes_per_page(&self) -> &ReservedBytesPerPage {
    &self.reserved_bytes_per_page
  }

  pub fn io(&self) -> &SqliteIo {
    &self.io
  }
}
