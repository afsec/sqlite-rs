use std::{fs::File, io::Read};

use sqlite_rs::{header::SqliteHeader, result::SQLiteResult};

fn main() -> SQLiteResult<()> {
  println!("SQLite info\n");

  let mut f = File::open("flights.db")?;
  let mut sqlite_header_buffer: [u8; 100] = [0; 100];

  let read_len = f.read(&mut sqlite_header_buffer)?;
  println!("Read {read_len} bytes.");

  print_hexdump(&sqlite_header_buffer[..])?;

  let sqlite_header = SqliteHeader::try_from(&sqlite_header_buffer)?;

  dbg!(&sqlite_header, &sqlite_header.page_size().get());

  Ok(())
}

fn print_hexdump(bytes: &[u8]) -> SQLiteResult<()> {
  use hexyl::{BorderStyle, PrinterBuilder};
  use std::io;

  let stdout = io::stdout();
  let mut handle = stdout.lock();
  let mut printer = PrinterBuilder::new(&mut handle)
    .show_color(true)
    .show_char_panel(true)
    .show_position_panel(true)
    .with_border_style(BorderStyle::Unicode)
    .enable_squeezing(false)
    .num_panels(2)
    .group_size(1)
    .build();
  printer.print_all(&bytes[..])?;
  Ok(())
}
