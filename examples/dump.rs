use std::{fs::File, io::Read};

use sqlite_rs::header::SqliteHeader;

type AppResult<T> = anyhow::Result<T>;

fn main() -> AppResult<()> {
  App::run()?;
  Ok(())
}

struct App;

impl App {
  fn run() -> AppResult<()> {
    println!("SQLite info\n");

    let mut f = File::open("flights.db")?;
    let mut sqlite_header_buffer: [u8; 100] = [0; 100];

    let read_len = f.read(&mut sqlite_header_buffer)?;
    println!("Read {read_len} bytes.");

    // Self::print_hexdump(&sqlite_header_buffer[..])?;

    let sqlite_header = SqliteHeader::try_from(&sqlite_header_buffer)?;

    Self::print_sqlite_info(&sqlite_header)?;

    println!("{sqlite_header:?}");
    Ok(())
  }
  fn print_sqlite_info(sqlite_header: &SqliteHeader) -> AppResult<()> {
    const LABEL_WIDTH: usize = 21;

    let mut output = "".to_owned();
    output.push_str("SQLite Header\n");
    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "database page size:",
      value = **sqlite_header.page_size()
    ));
    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "write format:",
      value = **sqlite_header.file_format_version_numbers().write_version()
    ));
    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "read format:",
      value = **sqlite_header.file_format_version_numbers().read_version()
    ));
    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "reserved bytes:",
      value = **sqlite_header.reserved_bytes_per_page()
    ));
    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "file change counter:",
      value = **sqlite_header.file_change_counter()
    ));

    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "database page count:",
      value = **sqlite_header.db_filesize_in_pages()
    ));

    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "freelist page count:",
      value = **sqlite_header.freelist_pages().total()
    ));

    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "schema cookie:",
      value = **sqlite_header.schema_cookie()
    ));

    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "schema format:",
      value = u32::from(sqlite_header.schema_format())
    ));

    println!("{output}");
    Ok(())
  }

  fn print_hexdump(bytes: &[u8]) -> AppResult<()> {
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
