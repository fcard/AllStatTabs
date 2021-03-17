use std::collections::HashMap;
use std::io;
use std::fs::*;
use std::io::prelude::*;
use std::str::*;

pub struct KeyValuePairs {
  kv : HashMap<String, i64>,
}

pub struct CharsPeekable<'a> {
  pub cs: Chars<'a>,
  pub peek: Option<char>,
}

impl<'a> CharsPeekable<'a> {
  fn new(cs: Chars<'a>) -> Self {
    Self {
      cs,
      peek: None
    }
  }

  fn peek(&mut self) -> Option<char> {
    if self.peek.is_none() {
      self.peek = self.cs.next()
    }
    self.peek
  }

  fn next(&mut self) -> Option<char> {
    if self.peek.is_some() {
      let ch = self.peek.unwrap();
      self.peek = None;
      Some(ch)
    } else {
      self.next()
    }
  }
}

fn keypart(ch: char) -> bool {
  ch.is_ascii_alphanumeric() || ch == '_'
}

fn valuepart(ch: char) -> bool {
  ch.is_digit(10) || ch == '_'
}


impl KeyValuePairs {
  pub fn get_bool(&self, key: &str, default: bool) -> bool {
    self.kv.get(key).map(|x| *x != 0).unwrap_or(default)
  }

  pub fn get_signed_byte(&self, key: &str, default: i8) -> i8 {
    self.kv.get(key).map(|x| *x as i8).unwrap_or(default)
  }

  pub fn from_file(filename: &str) -> io::Result<Self> {
    let mut file = File::open(filename)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    let mut new = Self::new();
    let mut iter = CharsPeekable::new(contents.chars());
    new.parse(&mut iter);
    Ok(new)
  }

  pub fn new() -> Self {
    Self {
      kv: HashMap::new()
    }
  }

  pub fn parse(&mut self, chars: &mut CharsPeekable) {
    while chars.peek().is_some() {
      match Self::parse1(chars) {
        Ok((key, value)) => {
          self.kv.insert(key, value);
        },

        Err(error) => {
          eprintln!("Error while parsing a All Stat Tabs config file: {}", error);
          Self::read_until_newline(chars);
        }
      }
    }
  }

  pub fn parse1(chars: &mut CharsPeekable) -> io::Result<(String, i64)> {
    Self::ignore_spaces(chars);
    let key = Self::read_key(chars)?;
    Self::ignore_spaces(chars);
    Self::read_equals(&key, chars)?;
    Self::ignore_spaces(chars);
    let value = Self::read_value(&key, chars)?;
    Self::ignore_spaces(chars);
    Ok((key, value))
  }

  pub fn ignore_spaces(chars: &mut CharsPeekable) {
    while let Some(ch) = chars.peek() {
      if ch == ' ' || ch == '\n' {
        chars.next();
      } else {
        break;
      }
    }
  }

  pub fn read_until_newline(chars: &mut CharsPeekable) {
    while chars.peek().unwrap_or('\n') != '\n' {
      chars.next();
    }
  }

  pub fn read_key(chars: &mut CharsPeekable) -> io::Result<String> {
    let mut key_str = String::new();
    while keypart(chars.peek().unwrap_or('\n')) {
      key_str.push(chars.next().unwrap())
    }
    if key_str.is_empty() {
      Err(io::Error::new(
        io::ErrorKind::Other,
        format!("Expected a number after key '{}' at the start of the line", &key_str))
      )
    } else {
      Ok(key_str)
    }
  }

  pub fn read_value(key: &str, chars: &mut CharsPeekable) -> io::Result<i64> {
    let mut value_str = String::new();
    if chars.peek().unwrap_or('\n') == '-' {
      value_str.push(chars.next().unwrap());
    }
    while valuepart(chars.peek().unwrap_or('\n')) {
      value_str.push(chars.next().unwrap())
    }
    if value_str.is_empty() {
      Err(io::Error::new(
        io::ErrorKind::Other,
        format!("Expected a number after 'key = ', where key = '{}'", key))
      )
    } else {
      Ok(value_str.parse::<i64>().unwrap())
    }
  }

  pub fn read_equals(key: &str, chars: &mut CharsPeekable) -> io::Result<()> {
    if chars.next() == Some('=') {
      Ok(())
    } else {
      Err(io::Error::new(
        io::ErrorKind::Other,
        format!("Expected a = between a key '{}' and value", key))
      )
    }
  }
}


