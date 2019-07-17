use std::borrow::{Borrow, ToOwned};
use std::fmt::{self, Display, Formatter};
use std::ops::Deref;
use std::str::{self, Utf8Error};
use std::string::String as StdString;

use crate::charset::private::Sealed;
use crate::charset::{
    Character as CharacterTrait, Charset as CharsetTrait, Str as StrTrait, String as StringTrait,
};

type StdStr = str;

#[derive(Debug)]
pub struct Charset;

impl CharsetTrait for Charset {
    type Alias = Alias;
    type Character = Character;
    type DecodeError = DecodeError;
    type Str = Str;
    type String = String;

    const MIB_ENUM: u16 = 1;
    const PREFERRED_MIME_NAME: Option<&'static UsAsciiStr> = None;
    const PRIMARY_NAME: &'static UsAsciiStr =
        unsafe { UsAsciiStr::from_bytes_unchecked(b"US-ASCII") };
}

impl Sealed for Charset {}

#[derive(Clone, Copy)]
pub struct Character(char);

impl CharacterTrait for Character {}

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Str(StdStr);

impl AsRef<[u8]> for Str {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl Display for Str {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl Sealed for Str {}

impl StrTrait for Str {
    type DecodeError = DecodeError;
    type String = String;

    fn decode(value: &[u8]) -> Result<&Self, Self::DecodeError> {
        let value = str::from_utf8(value)?;
        Ok(unsafe { &*(value as *const StdStr as *const Str) })
    }

    unsafe fn decode_unchecked(value: &[u8]) -> &Self {
        &*(value as *const [u8] as *const Str)
    }
}

impl ToOwned for Str {
    type Owned = String;

    fn to_owned(&self) -> Self::Owned {
        String(self.0.to_owned())
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct String(StdString);

impl AsRef<[u8]> for String {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsRef<Str> for String {
    fn as_ref(&self) -> &Str {
        &self
    }
}

impl Borrow<Str> for String {
    fn borrow(&self) -> &Str {
        &self
    }
}

impl Deref for String {
    type Target = Str;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(&*self.0 as *const StdStr as *const Str) }
    }
}

impl Display for String {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let borrow: &Str = self.borrow();
        borrow.fmt(formatter)
    }
}

impl From<String> for Vec<u8> {
    fn from(value: String) -> Self {
        value.0.into_bytes()
    }
}

impl Sealed for String {}

impl StringTrait for String {
    type DecodeError = DecodeError;
    type Str = Str;

    fn decode(value: Vec<u8>) -> Result<Self, (Vec<u8>, Self::DecodeError)> {
        let value = StdString::from_utf8(value).map_err(|error| {
            let decode_error = error.utf8_error();
            (error.into_bytes(), decode_error)
        })?;

        Ok(String(value))
    }

    unsafe fn decode_unchecked(value: Vec<u8>) -> Self {
        String(StdString::from_utf8_unchecked(value))
    }
}

pub type DecodeError = Utf8Error;

aliases! {
    Alias,

    (CsUtf8, b"csUTF8");
}
