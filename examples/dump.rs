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

  dbg!(
    &sqlite_header.magic_header_string().get(),
    &sqlite_header.page_size().get(),
    &sqlite_header.file_format_version_numbers().read_version(),
    &sqlite_header.file_format_version_numbers().write_version(),
    &sqlite_header.reserved_bytes_per_page().get(),
    &sqlite_header.payload_fractions().minimum().get(),
    &sqlite_header.payload_fractions().maximum().get(),
    &sqlite_header.payload_fractions().leaf().get(),
    &sqlite_header.file_change_counter().get(),
    &sqlite_header.db_filesize_in_pages().get()
  );
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
