#![no_std]
#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::dbg_macro,
    clippy::type_complexity,
    clippy::todo,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::mem_forget,
    clippy::unused_self,
    clippy::filter_map_next,
    clippy::needless_continue,
    clippy::needless_borrow,
    clippy::match_wildcard_for_single_variants,
    clippy::if_let_mutex,
    clippy::mismatched_target_os,
    clippy::await_holding_lock,
    clippy::match_on_vec_items,
    clippy::imprecise_flops,
    clippy::suboptimal_flops,
    clippy::lossy_float_literal,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::fn_params_excessive_bools,
    clippy::exit,
    clippy::inefficient_to_string,
    clippy::linkedlist,
    clippy::macro_use_imports,
    clippy::option_option,
    clippy::verbose_file_reads,
    clippy::unnested_or_patterns,
    clippy::str_to_string,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
    missing_debug_implementations,
    // missing_docs
)]
#![deny(unreachable_pub, private_in_public)]
#![allow(
  elided_lifetimes_in_paths,
  clippy::new_ret_no_self,
  clippy::unused_self
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(test, allow(clippy::float_cmp))]

use result::{SQLiteError, SQLiteResult};

use crate::header::SqliteHeader;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod header;
pub mod result;
#[macro_use]
pub mod macros;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct SQLiteDatabase {
  mode: Mode,
  header: SqliteHeader,
  // pages: &'a [u8],
  pager: SqlitePager,
}

#[derive(Debug)]
pub enum Mode {
  InMemoryNoStd,
  Std,
}
impl SQLiteDatabase {
  const MINIMUM_USABLE_SIZE: usize = 480; // TODO: TBD
  pub const MINIMUM_SIZE: usize =
    SqliteHeader::LENGTH_BYTES + Self::MINIMUM_USABLE_SIZE;

  pub fn new_in_memory(bytes: &[u8]) -> SQLiteResult<()> {
    let mode = Mode::InMemoryNoStd;
    let header = SqliteHeader::try_from(bytes)?;
    // let pages = &bytes[SqliteHeader::LENGTH_BYTES..];
    let pager = SqlitePager;

    let database = SQLiteDatabase {
      mode,
      header,
      // pages,
      pager,
    };

    std::dbg!(database);

    todo!();
    Ok(())
  }

  #[cfg(not(feature = "std"))]
  pub fn new() {
    todo!()
  }
}

// impl<'a> TryFrom<&'a [u8]> for SQLiteDatabase<'a> {
//   type Error = SQLiteError;

//   fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
//     Ok(Self {
//       mode: Mode::InMemoryNoStd,
//       header: SqliteHeader::try_from(data)?,
//       pages,
//       pager: SqlitePager,
//     })
//   }
// }

#[derive(Debug)]
pub struct SqlitePager;
