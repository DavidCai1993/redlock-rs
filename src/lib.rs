//! A rust redlock implementation for distributed, highly-available redis locks.

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate quick_error;
extern crate redis;
extern crate rand;

pub use self::errors::RedlockResult;
pub use self::redlock::{Lock, Redlock, Config};

mod errors;
mod scripts;
mod redlock;
mod util;
