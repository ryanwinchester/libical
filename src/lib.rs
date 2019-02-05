#[macro_use] mod macros;

pub mod errors;
pub type KhResult<T> = Result<T,errors::KhError>;

pub mod actions;
pub mod backup;
pub mod khline;
pub mod calendars;
pub mod config;
pub mod cursorfile;
pub mod defaults;
pub mod edit;
pub mod icalwrap;
pub mod input;
pub mod selectors;
pub mod seqfile;
pub mod utils;
#[cfg(test)] pub mod testutils;
#[cfg(test)] pub mod testdata;

#[cfg(test)] use assert_fs;
#[cfg(test)] use predicates;
#[cfg(test)] #[macro_use] extern crate maplit;
#[cfg(test)] #[macro_use] extern crate pretty_assertions;

use atty;
use backtrace;
use chrono;
use dirs;
use fs2;
use itertools;
use libc;
use ical;
use stderrlog;
use tempfile;
use uuid;
use walkdir;
use yansi;
use toml;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;
#[macro_use] extern crate indoc;
#[macro_use] extern crate lazy_static;
