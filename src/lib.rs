//! Stringly is a human readable and writable serialization format that is not
//! self-describing.
//!
//! This crate provides serialization into and deserialization from the
//! Stringly format using the [Serde](https://serde.rs) framework.
//!
//! # Format
//!
//! *   Strings are serialized as UTF-8 strings, without enclosing in quotes or
//!     some form of escaping.
//!
//!     ```
//!     # macro_rules! check { ($v:expr => $s:literal) => { check(&$v, $s) } }
//!     # pub fn check<'a, V: PartialEq + std::fmt::Debug + serde::Serialize + serde::Deserialize<'a>>(v: &V, s: &'a str) {
//!     #   assert_eq!(stringly::to_string(v).unwrap(), s);
//!     #   assert_eq!(stringly::from_str::<'a, V>(s).unwrap(), *v);
//!     # }
//!     # check!(
//!     "Example" => "Example"
//!     # );
//!     ```
//!
//! *   Numbers are serialized by the usual string representation (like in
//!     JSON).
//!
//!     ```
//!     # macro_rules! check { ($v:expr => $s:literal) => { check(&$v, $s) } }
//!     # pub fn check<'a, V: PartialEq + std::fmt::Debug + serde::Serialize + serde::Deserialize<'a>>(v: &V, s: &'a str) {
//!     #   assert_eq!(stringly::to_string(v).unwrap(), s);
//!     #   assert_eq!(stringly::from_str::<'a, V>(s).unwrap(), *v);
//!     # }
//!     # check!(
//!     1i32 => "1"
//!     # ); check!(
//!     1.2f64 => "1.2"
//!     # );
//!     ```
//!
//! *   A sequence of items is serialized by joining serialized items with `,`.
//!     A serialized item containing a `,` is [protected][protection] by
//!     enclosing the serialized item in curly braces. Otherwise a `,`
//!     separating two items would be indistinguishable from a `,` inside an
//!     item.
//!
//!     ```
//!     # macro_rules! check { ($v:expr => $s:literal) => { check(&$v, $s) } }
//!     # pub fn check<'a, V: PartialEq + std::fmt::Debug + serde::Serialize + serde::Deserialize<'a>>(v: &V, s: &'a str) {
//!     #   assert_eq!(stringly::to_string(v).unwrap(), s);
//!     #   assert_eq!(stringly::from_str::<'a, V>(s).unwrap(), *v);
//!     # }
//!     # check!(
//!     vec![1i32, 2i32] => "1,2"
//!     # ); check!(
//!     vec!["1,2", "3", "4"] => "{1,2},3,4"
//!     # );
//!     ```
//!
//! *   A mapping is serialized by `,`-joining serialized pairs, where each
//!     pair is serialized by joining the serialized key and value with a `=`.
//!     A serialized key is protected for `=` and `,` and a serialized value
//!     for `,`.
//!
//!     ```
//!     # macro_rules! check { ($v:expr => $s:literal) => { check(&$v, $s) } }
//!     # pub fn check<'a, V: PartialEq + std::fmt::Debug + serde::Serialize + serde::Deserialize<'a>>(v: &V, s: &'a str) {
//!     #   assert_eq!(stringly::to_string(v).unwrap(), s);
//!     #   assert_eq!(stringly::from_str::<'a, V>(s).unwrap(), *v);
//!     # }
//!     let mut v = std::collections::BTreeMap::new();
//!     v.insert("a", 1i32);
//!     v.insert("b", 2i32);
//!     # check!(
//!     v => "a=1,b=2"
//!     # );
//!     ```
//!
//! [protection]: #protection
//!
//! ## Protection
//!
//! Instead of escaping special characters e.g. by placing a backslash in front
//! of characters that need escaping, the Stringly format requires the entire
//! string containing special characters to be enclosed in curly braces.
//! Unprotecting a string involves detecting and slicing off the curly braces.
//! This has the following consequences: strings starting with `{` and ending
//! with `}` must be protected regardless the presence of special characters,
//! or the braces will be sliced off unintentionally, and strings with
//! unbalanced braces must be balanced and protected, or trailing special
//! characters will be ignored.
//!
//! Note that protection is applied just in time. When serializing a `str`, no
//! protection will be applied, simply because the serialization does not have
//! special characters. When serializing a `Vec<str>` all items must be
//! protected for `,`, because `,` is used as special character to separate
//! the items.
//!
//! Formally, a serialized string is protected by prepending `'{'` or `'{<' +
//! '{'*l + >'` and appending `'}'` or `'<' + '}'*r + '>}'`, with `l` and `r`
//! nonnegative numbers such that the protected string does not have negative
//! curly scopes.
//!
//! # Examples
//!
//! ```
//! use serde::{Serialize,Deserialize};
//!
//! #[derive(Serialize, Deserialize, Debug, PartialEq)]
//! struct Example {
//!     a: i32,
//!     b: String,
//! }
//!
//! let v = Example { a: 1, b: "2".to_string() };
//! let s = "a=1,b=2";
//! assert_eq!(stringly::to_string(&v).unwrap(), s);
//! assert_eq!(stringly::from_str::<Example>(&s).unwrap(), v);
//! ```

extern crate serde;

mod de;
mod error;
mod ser;
pub mod util;

pub use de::{from_str, Deserializer};
pub use error::{Error, Result};
pub use ser::{to_string, Serializer};

#[cfg(test)]
mod tests;
