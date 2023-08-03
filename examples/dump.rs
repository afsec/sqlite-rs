use std::{fs::File, io::Read};

use sqlite_rs::{header::SqliteHeader, result::SQLiteResult};

fn main() -> SQLiteResult<()> {
    println!("SQLite info\n");

    let mut f = File::open("flights.db")?;
    let mut buffer: [u8; 100] = [0; 100];

    let read_len = f.read(&mut buffer)?;
    println!("Read {read_len} bytes.");

    print_hexdump(&buffer[..])?;

    let sqlite_header = SqliteHeader::try_from(&buffer)?;
    dbg!(&sqlite_header);
    
    // dbg!(&sqlite_header, &sqlite_header.page_size().get());

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
