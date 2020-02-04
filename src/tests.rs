use crate::de::{from_str, from_str_pretty};
use crate::error::{Error, Result};
use crate::ser::{to_string, to_string_pretty};
use crate::util;
use core::fmt::Debug;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

fn assert_serde<'a, T: PartialEq + Debug + Serialize + Deserialize<'a>>(value: &T, serial: &'a str) {
  assert_eq!(to_string(value).unwrap(), serial);
  assert_eq!(from_str::<'a, T>(serial).unwrap(), *value);
}

fn assert_serde_pretty<T: PartialEq + Debug + Serialize + DeserializeOwned>(value: &T, serial: &str) {
  assert_eq!(to_string_pretty(value).unwrap(), serial);
  assert_eq!(from_str_pretty::<T>(serial).unwrap(), *value);
}

macro_rules! assert_serde {
  ($value:expr, $serial:literal) => {
    assert_serde(&$value, $serial);
  };
  ($value:expr, $serial:literal, $pretty:literal) => {
    assert_serde(&$value, $serial);
    assert_serde_pretty(&$value, $pretty);
    assert_eq!(util::prettify($serial), $pretty);
    assert_eq!(util::deprettify($pretty).unwrap(), $serial);
  };
}

macro_rules! assert_de {
  ($value:expr, $serial:literal) => {
    assert_eq!($value, from_str($serial).unwrap());
  };
}

macro_rules! assert_de_error {
  ($T:ty, $serial:literal, $err:expr) => {
    assert_eq!(from_str($serial) as Result<$T>, Err($err));
  };
}

#[test]
fn test_bool() {
  assert_serde!(true, "True", "True\n");
  assert_de!(true, "true");
  assert_de!(true, "yes");
  assert_de!(true, "YES");
  assert_serde!(false, "False", "False\n");
  assert_de!(false, "false");
  assert_de!(false, "no");
  assert_de!(false, "NO");
}

#[test]
fn test_integer() {
  assert_serde!(1i8, "1", "1\n");
  assert_serde!(2i16, "2", "2\n");
  assert_serde!(3i32, "3", "3\n");
  assert_serde!(4i64, "4", "4\n");
  assert_serde!(1u8, "1", "1\n");
  assert_serde!(2u16, "2", "2\n");
  assert_serde!(3u32, "3", "3\n");
  assert_serde!(4u64, "4", "4\n");
  assert_de_error!(i32, "1.", Error::NotAnInteger);
  assert_de_error!(i32, "1a", Error::NotAnInteger);
}

#[test]
fn test_float() {
  assert_serde!(1f32, "1", "1\n");
  assert_serde!(2f64, "2", "2\n");
  assert_de_error!(f32, "1a", Error::NotAFloatingPointNumber);
}

#[test]
fn test_char() {
  assert_serde!('a', "a", "a\n");
  assert_de_error!(char, "ab", Error::NotASingleCharacter);
  assert_de_error!(char, "", Error::NotASingleCharacter);
}

#[test]
fn test_str() {
  assert_serde!("abc", "abc");
}

#[test]
fn test_string() {
  assert_serde!("abc".to_string(), "abc", "abc\n");
}

#[test]
fn test_option() {
  assert_serde!(Some("1"), "1");
  assert_serde!(Some(""), "{}");
  assert_serde!(Some("{}"), "{{}}");
  assert_serde!(Some("{"), "{");
  assert_serde!(Some("}"), "}");
  assert_serde!(Some(1i32), "1");
  assert_serde!(None as Option<i32>, "");
}

#[test]
fn test_tuple() {
  assert_serde!((), "", "");
  assert_de_error!((), "a", Error::UnexpectedValueForUnit);

  assert_serde!(("".to_string(),), "{}", "{}\n");
  assert_serde!((1i32,), "1", "1\n");
  assert_de_error!((i32,), "1,2", Error::TooManyElements);

  assert_serde!(("".to_string(), "".to_string()), "{},{}", "{}\n{}\n");
  assert_serde!((1i32, 2f64), "1,2", "1\n2\n");
  assert_serde!(("{", "}"), "{{<}>},{<{>}}");
  assert_serde!(("{".to_string(), "}".to_string()), "{{<}>},{<{>}}", "{{<}>}\n{<{>}}\n");
}

#[test]
fn test_struct() {
  #[derive(Debug, PartialEq, Serialize, Deserialize)]
  struct S;

  assert_serde!(S, "", "");
  assert_de_error!(S, "a", Error::UnexpectedValueForUnit);

  #[derive(Debug, PartialEq, Serialize, Deserialize)]
  struct Ss(String);

  assert_serde!(Ss("".to_string()), "", "");
  assert_serde!(Ss("1".to_string()), "1", "1\n");

  #[derive(Debug, PartialEq, Serialize, Deserialize)]
  struct Sss(String, String);

  assert_serde!(Sss("".to_string(), "".to_string()), "{},{}", "{}\n{}\n");
  assert_serde!(Sss("1".to_string(), "2".to_string()), "1,2", "1\n2\n");

  #[derive(Debug, PartialEq, Serialize, Deserialize)]
  struct Si(i32);

  assert_serde!(Si(1), "1", "1\n");
  assert_de_error!(Si, "1,2", Error::NotAnInteger);

  #[derive(Debug, PartialEq, Serialize, Deserialize)]
  struct Sif(i32, f64);

  assert_serde!(Sif(1, 1.), "1,1", "1\n1\n");
  assert_de_error!(Sif, "1,2,3", Error::TooManyElements);
}

#[test]
fn test_enum() {
  #[derive(Debug, PartialEq, Serialize, Deserialize)]
  enum Test {
    A,
    B(String),
    C(i32),
    D(String, String),
    E(i32, f64),
  }

  assert_serde!(Test::A, "A", "A\n");
  assert_de_error!(Test, "A{a}", Error::UnexpectedValueForUnit);

  assert_serde!(Test::B("".to_string()), "B", "B\n");
  assert_serde!(Test::B("1".to_string()), "B{1}", "B\n  1\n");
  assert_serde!(Test::C(1), "C{1}", "C\n  1\n");
  assert_de_error!(Test, "C{1,2}", Error::NotAnInteger);

  assert_serde!(Test::D("".to_string(), "".to_string()), "D{{},{}}", "D\n  {}\n  {}\n");
  assert_serde!(Test::D("1".to_string(), "2".to_string()), "D{1,2}", "D\n  1\n  2\n");
  assert_serde!(Test::D("{".to_string(), "}".to_string()), "D{{{<}>},{<{>}}}", "D\n  {{<}>}\n  {<{>}}\n");
  assert_serde!(Test::E(1, 2.), "E{1,2}", "E\n  1\n  2\n");
  assert_de_error!(Test, "E{1,2,3}", Error::TooManyElements);
}

#[test]
fn test_seq() {
  assert_serde!(Vec::new() as Vec<i32>, "", "");
  assert_serde!(vec![""] as Vec<&str>, "{}");
  assert_serde!(vec!["", ""] as Vec<&str>, "{},{}");
  assert_serde!(vec!["".to_string()], "{}", "{}\n");
  assert_serde!(vec!["".to_string(), "".to_string()], "{},{}", "{}\n{}\n");
  assert_serde!(vec![1i32], "1", "1\n");
  assert_serde!(vec![1i32, 2i32, 3i32], "1,2,3", "1\n2\n3\n");
}

#[cfg(feature = "std")]
#[test]
fn test_map() {
  let mut m1 = std::collections::BTreeMap::new();
  assert_serde!(m1, "", "");
  m1.insert("A".to_string(), "B".to_string());
  assert_serde!(m1, "A=B", "A=B\n");
  m1.insert("C".to_string(), "D".to_string());
  assert_serde!(m1, "A=B,C=D", "A=B\nC=D\n");

  let mut m2 = std::collections::BTreeMap::new();
  assert_serde!(m2, "", "");
  m2.insert("A".to_string(), vec![]);
  assert_serde!(m2, "A=", "A=\n");
  m2.insert("B".to_string(), vec![1i32]);
  assert_serde!(m2, "A=,B=1", "A=\nB=1\n");
  m2.insert("C".to_string(), vec![2i32, 3i32]);
  assert_serde!(m2, "A=,B=1,C={2,3}", "A=\nB=1\nC=\n  2\n  3\n");
}
