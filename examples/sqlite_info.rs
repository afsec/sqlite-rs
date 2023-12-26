// use sqlite_rs::SqliteConnection;
// use std::{fs::File, io::Read};

use sqlite_rs::SqliteConnection;

type AppResult<T> = Result<T, AppError>;
type AppError = Box<dyn std::error::Error>;

fn main() -> AppResult<()> {
  SqliteInfo::run()?;
  Ok(())
}

struct SqliteInfo;

impl SqliteInfo {
  fn run() -> AppResult<()> {
    let files = [
      "./data/flights-initial.db",
      "./data/flights-populated.db",
      "./data/flights-deleted.db",
      "./data/mydatabase.db",
    ];
    files.iter().try_for_each(|file_path| -> AppResult<()> {
      let conn = SqliteConnection::open(format!("sqlite://{file_path}"))?;
      // let sqlite_database = Self::read_bytes(&file)?;

      println!("[{file_path}]:");
      Self::print_sqlite_info(&conn)?;
      Ok(())
    })?;

    Ok(())
  }

  fn print_sqlite_info(conn: &SqliteConnection) -> AppResult<()> {
    const LABEL_WIDTH: usize = 21;

    // TODO:
    let sqlite_header = conn.runtime().header();

    let mut output = "".to_owned();

    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "database page size:",
      value = u32::from(sqlite_header.page_size())
    ));
    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "write format:",
      value =
        u8::from(sqlite_header.file_format_version_numbers().write_version())
    ));
    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "read format:",
      value =
        u8::from(sqlite_header.file_format_version_numbers().read_version())
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

    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "default cache size:",
      value = **sqlite_header.suggested_cache_size()
    ));

    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "autovacuum top root:",
      value = **sqlite_header
        .incremental_vacuum_settings()
        .largest_root_btree_page()
    ));

    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "incremental vacuum:",
      value = u32::from(
        sqlite_header
          .incremental_vacuum_settings()
          .incremental_vacuum_mode()
      )
    ));

    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "text encoding:",
      value = sqlite_header.database_text_encoding()
    ));

    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "user version:",
      value = **sqlite_header.user_version()
    ));

    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "application id:",
      value = **sqlite_header.application_id()
    ));

    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "software version:",
      value = **sqlite_header.write_library_version()
    ));

    println!("{output}");
    Ok(())
  }
}
