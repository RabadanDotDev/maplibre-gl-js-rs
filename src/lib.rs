#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
// We work with JsFutures, which are not send due to JavaScript being inherently
// single-threaded. Therefore, we need to allow this.
#![allow(clippy::future_not_send)]

#[cfg(test)]
pub mod test_utils;
