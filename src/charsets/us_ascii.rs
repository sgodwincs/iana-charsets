use std::borrow::{Borrow, ToOwned};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::str;

use crate::charset::private::Sealed;
use crate::charset::{
    Character as CharacterTrait, Charset as CharsetTrait, Str as StrTrait, String as StringTrait,
};

#[derive(Debug)]
pub struct Charset;

impl CharsetTrait for Charset {
    type Alias = Alias;
    type Character = Character;
    type DecodeError = DecodeError;
    type Str = Str;
    type String = String;

    const MIB_ENUM: u16 = 1;
    const PREFERRED_MIME_NAME: Option<&'static Str> =
        Some(unsafe { Str::from_bytes_unchecked(b"US-ASCII") });
    const PRIMARY_NAME: &'static Str = unsafe { Str::from_bytes_unchecked(b"US-ASCII") };
}

#[derive(Clone, Copy)]
pub struct Character(u8);

impl CharacterTrait for Character {}

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Str([u8]);

impl Str {
    pub const unsafe fn from_bytes_unchecked(value: &[u8]) -> &Self {
        &*(value as *const [u8] as *const Str)
    }
}

impl AsRef<[u8]> for Str {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Display for Str {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        unsafe { str::from_utf8_unchecked(&self.0) }.fmt(formatter)
    }
}

impl Sealed for Str {}

impl StrTrait for Str {
    type DecodeError = DecodeError;
    type String = String;

    fn decode(value: &[u8]) -> Result<&Self, Self::DecodeError> {
        validate(value)?;
        Ok(unsafe { Self::decode_unchecked(value) })
    }

    unsafe fn decode_unchecked(value: &[u8]) -> &Self {
        Self::from_bytes_unchecked(value)
    }
}

impl ToOwned for Str {
    type Owned = String;

    fn to_owned(&self) -> Self::Owned {
        String(self.0.to_vec())
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct String(Vec<u8>);

impl AsRef<[u8]> for String {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsRef<Str> for String {
    fn as_ref(&self) -> &Str {
        self.borrow()
    }
}

impl Borrow<Str> for String {
    fn borrow(&self) -> &Str {
        unsafe { Str::from_bytes_unchecked(&*self.0) }
    }
}

impl Display for String {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let borrow: &Str = self.borrow();
        borrow.fmt(formatter)
    }
}

impl Sealed for String {}

impl StringTrait for String {
    type DecodeError = DecodeError;
    type Str = Str;

    fn decode(value: Vec<u8>) -> Result<Self, Self::DecodeError> {
        validate(&value)?;
        Ok(unsafe { Self::decode_unchecked(value) })
    }

    unsafe fn decode_unchecked(value: Vec<u8>) -> Self {
        String(value)
    }
}

#[derive(Debug)]
pub struct DecodeError;

impl Display for DecodeError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "invalid US-ASCII")
    }
}

impl Error for DecodeError {}

fn validate(value: &[u8]) -> Result<(), DecodeError> {
    for byte in value {
        if !byte.is_ascii() {
            return Err(DecodeError);
        }
    }

    Ok(())
}

aliases! {
    Alias,

    (AnsiX3_4_1968, b"ANSI_X3.4-1968");
    (AnsiX3_4_1986, b"ANSI_X3.4-1986");
    (Cp367, b"cp367");
    (CsAscii, b"csAscii");
    (Ibm367, b"IBM367");
    (Iso646Irv1991, b"ISO_646.irv:1991");
    (Iso646Us, b"ISO646-US");
    (IsoIr6, b"iso-ir-6");
    (UsAscii, b"US-ASCII");
    (Us, b"us");
}
