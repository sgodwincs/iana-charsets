use std::borrow::{Borrow, ToOwned};
use std::str::{self, Utf8Error};
use std::string::String as StdString;

use crate::charset::{
    Character as CharacterTrait, Charset as CharsetTrait, Str as StrTrait, String as StringTrait,
};

type StdStr = str;

#[derive(Debug)]
pub struct Charset;

impl Charset {
    pub const unsafe fn from_bytes_unchecked(value: &[u8]) -> &<Self as CharsetTrait>::Str {
        &*(value as *const [u8] as *const Str)
    }
}

impl CharsetTrait for Charset {
    type Alias = Alias;
    type Character = Character;
    type DecodeError = DecodeError;
    type Str = Str;
    type String = String;

    const MIB_ENUM: u16 = 1;
    const PREFERRED_MIME_NAME: Option<&'static UsAsciiStr> = None;
    const PRIMARY_NAME: &'static UsAsciiStr =
        unsafe { UsAsciiCharset::from_bytes_unchecked(b"US-ASCII") };

    unsafe fn decode_unchecked(value: &[u8]) -> &Self::Str {
        Self::from_bytes_unchecked(value)
    }

    fn validate(value: &[u8]) -> Result<(), DecodeError> {
        str::from_utf8(value).map(|_| ())
    }
}

#[derive(Clone, Copy)]
pub struct Character(char);

impl CharacterTrait for Character {}

pub struct Str(StdStr);

impl StrTrait<String> for Str {}

impl ToOwned for Str {
    type Owned = String;

    fn to_owned(&self) -> Self::Owned {
        String(self.0.to_owned())
    }
}

#[derive(Clone)]
pub struct String(StdString);

impl Borrow<Str> for String {
    fn borrow(&self) -> &Str {
        unsafe { &*(&*self.0 as *const StdStr as *const Str) }
    }
}

impl StringTrait<Str> for String {}

pub type DecodeError = Utf8Error;

aliases! {
    Alias,

    (CsUtf8, b"csUTF8");
}
