use std::borrow::{Borrow, ToOwned};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult, Write};
use std::ops::Deref;

use crate::charset::private::Sealed;
use crate::charset::{
    Character as CharacterTrait, Charset as CharsetTrait, DecodeError as DecodeErrorTrait,
    Str as StrTrait, String as StringTrait,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Charset;

impl CharsetTrait for Charset {
    type Alias = Alias;
    type Character = Character;
    type DecodeError = DecodeError;
    type Str = Str;
    type String = String;

    const MIB_ENUM: u16 = 4;
    const PREFERRED_MIME_NAME: Option<&'static UsAsciiStr> =
        Some(unsafe { UsAsciiStr::from_bytes_unchecked(b"ISO-8859-1") });
    const PRIMARY_NAME: &'static UsAsciiStr =
        unsafe { UsAsciiStr::from_bytes_unchecked(b"ISO_8859-1:1987") };
}

impl Sealed for Charset {}

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Character(char);

impl CharacterTrait for Character {}

impl Debug for Character {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_char(self.0)
    }
}

impl Display for Character {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_char(self.0)
    }
}

impl From<u8> for Character {
    fn from(value: u8) -> Self {
        Character(value as char)
    }
}

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Str([u8]);

impl AsRef<[u8]> for Str {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl Debug for Str {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        for &byte in &self.0 {
            Debug::fmt(&Character::from(byte), formatter)?;
        }

        Ok(())
    }
}

impl Display for Str {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        Debug::fmt(self, formatter)
    }
}

impl Sealed for Str {}

impl StrTrait for Str {
    type DecodeError = DecodeError;
    type String = String;

    fn decode(value: &[u8]) -> Result<&Self, Self::DecodeError> {
        // Unsafe justification: ISO_8859-1:1987 is a 1-byte charset with a 1-to-1 mapping for each
        // 8-bit value to a character, so any byte slice can be considered valid.
        Ok(unsafe { Self::decode_unchecked(value) })
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

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct String(Vec<u8>);

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

impl Debug for String {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        Debug::fmt(&**self, formatter)
    }
}

impl Deref for String {
    type Target = <Self as StringTrait>::Str;

    fn deref(&self) -> &Self::Target {
        unsafe { Str::decode_unchecked(&*self.0) }
    }
}

impl Display for String {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        Debug::fmt(self, formatter)
    }
}

impl<'str> From<&'str Str> for String {
    fn from(value: &'str Str) -> Self {
        value.to_owned()
    }
}

impl From<String> for Vec<u8> {
    fn from(value: String) -> Self {
        value.0
    }
}

impl Sealed for String {}

impl StringTrait for String {
    type DecodeError = DecodeError;
    type Str = Str;

    fn decode(value: Vec<u8>) -> Result<Self, (Vec<u8>, Self::DecodeError)> {
        // Unsafe justification: ISO_8859-1:1987 is a 1-byte charset with a 1-to-1 mapping for each
        // 8-bit value to a character, so any byte slice can be considered valid.
        Ok(unsafe { Self::decode_unchecked(value) })
    }

    unsafe fn decode_unchecked(value: Vec<u8>) -> Self {
        String(value)
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct DecodeError;

impl DecodeErrorTrait for DecodeError {}

impl Display for DecodeError {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("invalid ISO-8859-1:1987 (ISO-8859-1)")
    }
}

impl Error for DecodeError {}

aliases! {
    Alias,

    (Cp819, b"CP819");
    (CsIsoLatin1, b"csISOLatin1");
    (Ibm819, b"IBM819");
    (Iso8859_1, b"ISO-8859-1");
    (Iso8859_1Alt, b"ISO_8859-1");
    (IsoIr100, b"iso-ir-100");
    (L1, b"l1");
    (Latin1, b"latin1");
}
