use crate::{
  header::{MagicHeaderString, PageSize},
  io::SqliteIo,
  result::SqliteResult,
  traits::ParseBytes,
};

#[derive(Debug)]
pub struct SqlitePager {
  io: SqliteIo,
  page_size: PageSize,
  cur_page_pos: usize,
  // btree_page_header: BtreePageHeader,
}

impl SqlitePager {
  pub fn connect(mut io: SqliteIo) -> SqliteResult<Self> {
    io.rewind()?;
    const BYTES_TO_READ: usize =
      MagicHeaderString::LENGTH_BYTES + PageSize::LENGTH_BYTES;
    let mut buf = [0u8; BYTES_TO_READ];

    io.read(&mut buf)?;

    Ok(Self {
      io,
      page_size: PageSize::parse_bytes(&buf[16..=17])?,
      cur_page_pos: 0,
    })
  }
  pub fn first(&mut self) -> SqliteResult<Page> {
    Ok(self.read(0)?)
  }

  pub fn read(&mut self, pos: u32) -> SqliteResult<Page> {
    let page_size = self.page_size().clone();
    let page_position = pos * u32::from(&page_size);
    self.io.seek(page_position.into())?;

    match page_size {
      PageSize::L512 => {
        const BUF_SIZE: usize = 512;
        let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
        // TODO: Write tests
        self.io.read(&mut buf)?;

        let mut final_buffer = [0u8; PAGE_MAX_LENGTH];
        for (idx, byte) in buf.iter().enumerate() {
          final_buffer[idx] = *byte;
        }
        Ok(Page {
          length: page_size,
          inner: Box::new(final_buffer),
        })
      }
      PageSize::L1024 => {
        const BUF_SIZE: usize = 1024;
        let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
        self.io.read(&mut buf)?;
        let mut final_buffer = [0u8; PAGE_MAX_LENGTH];
        for (idx, byte) in buf.iter().enumerate() {
          final_buffer[idx] = *byte;
        }
        Ok(Page {
          length: page_size,
          inner: Box::new(final_buffer),
        })
      }
      PageSize::L2048 => {
        const BUF_SIZE: usize = 2048;
        let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
        self.io.read(&mut buf)?;
        let mut final_buffer = [0u8; PAGE_MAX_LENGTH];
        for (idx, byte) in buf.iter().enumerate() {
          final_buffer[idx] = *byte;
        }
        Ok(Page {
          length: page_size,
          inner: Box::new(final_buffer),
        })
      }
      PageSize::L4096 => {
        const BUF_SIZE: usize = 4096;
        let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
        self.io.read(&mut buf)?;
        let mut final_buffer = [0u8; PAGE_MAX_LENGTH];
        for (idx, byte) in buf.iter().enumerate() {
          final_buffer[idx] = *byte;
        }
        Ok(Page {
          length: page_size,
          inner: Box::new(final_buffer),
        })
      }
      PageSize::L8192 => {
        const BUF_SIZE: usize = 8192;
        let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
        self.io.read(&mut buf)?;
        let mut final_buffer = [0u8; PAGE_MAX_LENGTH];
        for (idx, byte) in buf.iter().enumerate() {
          final_buffer[idx] = *byte;
        }
        Ok(Page {
          length: page_size,
          inner: Box::new(final_buffer),
        })
      }
      PageSize::L16384 => {
        const BUF_SIZE: usize = 16384;
        let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
        self.io.read(&mut buf)?;
        let mut final_buffer = [0u8; PAGE_MAX_LENGTH];
        for (idx, byte) in buf.iter().enumerate() {
          final_buffer[idx] = *byte;
        }
        Ok(Page {
          length: page_size,
          inner: Box::new(final_buffer),
        })
      }
      PageSize::L32768 => {
        const BUF_SIZE: usize = 32768;
        let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
        self.io.read(&mut buf)?;
        let mut final_buffer = [0u8; PAGE_MAX_LENGTH];
        for (idx, byte) in buf.iter().enumerate() {
          final_buffer[idx] = *byte;
        }
        Ok(Page {
          length: page_size,
          inner: Box::new(final_buffer),
        })
      }
      PageSize::L65536 => {
        const BUF_SIZE: usize = 65536;
        let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
        self.io.read(&mut buf)?;
        let mut final_buffer = [0u8; PAGE_MAX_LENGTH];
        for (idx, byte) in buf.iter().enumerate() {
          final_buffer[idx] = *byte;
        }
        Ok(Page {
          length: page_size,
          inner: Box::new(final_buffer),
        })
      }
    }
  }

  pub fn page_size(&self) -> &PageSize {
    &self.page_size
  }
}
const PAGE_MAX_LENGTH: usize = 65536;
type PageBuf = [u8; PAGE_MAX_LENGTH];

#[derive(Debug)]
pub struct Page {
  length: PageSize,
  inner: Box<PageBuf>,
}

impl Page {
  pub fn get(&self) -> &PageBuf {
    self.inner.as_ref()
  }
}
