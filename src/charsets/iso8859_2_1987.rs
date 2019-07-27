use std::borrow::{Borrow, ToOwned};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult, Write};
use std::ops::Deref;

use crate::charset::private::Sealed;
use crate::charset::{
    Character as CharacterTrait, Charset as CharsetTrait, DecodeError as DecodeErrorTrait,
    Str as StrTrait, String as StringTrait,
};

#[rustfmt::skip]
const GRAPHICS_RIGHT_TO_UNICODE_MAP: [char; 96] = [
    '\u{00a0}', '\u{0104}', '\u{02d8}', '\u{0141}', '\u{00a4}', '\u{013d}', '\u{015a}', '\u{00a7}', '\u{00a8}', '\u{0160}', '\u{015e}', '\u{0164}', '\u{0179}', '\u{00ad}', '\u{017d}', '\u{017b}',
    '\u{00b0}', '\u{0105}', '\u{02db}', '\u{0142}', '\u{00b4}', '\u{013e}', '\u{015b}', '\u{02c7}', '\u{00b8}', '\u{0161}', '\u{015f}', '\u{0165}', '\u{017a}', '\u{02dd}', '\u{017e}', '\u{017c}',
    '\u{0154}', '\u{00c1}', '\u{00c2}', '\u{0102}', '\u{00c4}', '\u{0139}', '\u{0106}', '\u{00c7}', '\u{010c}', '\u{00c9}', '\u{0118}', '\u{00cb}', '\u{011a}', '\u{00cd}', '\u{00ce}', '\u{010e}',
    '\u{0110}', '\u{0143}', '\u{0147}', '\u{00d3}', '\u{00d4}', '\u{0150}', '\u{00d6}', '\u{00d7}', '\u{0158}', '\u{016e}', '\u{00da}', '\u{0170}', '\u{00dc}', '\u{00dd}', '\u{0162}', '\u{00df}',
    '\u{0155}', '\u{00e1}', '\u{00e2}', '\u{0103}', '\u{00e4}', '\u{013a}', '\u{0107}', '\u{00e7}', '\u{010d}', '\u{00e9}', '\u{0119}', '\u{00eb}', '\u{011b}', '\u{00ed}', '\u{00ee}', '\u{010f}',
    '\u{0111}', '\u{0144}', '\u{0148}', '\u{00f3}', '\u{00f4}', '\u{0151}', '\u{00f6}', '\u{00f7}', '\u{0159}', '\u{016f}', '\u{00fa}', '\u{0171}', '\u{00fc}', '\u{00fd}', '\u{0163}', '\u{02d9}',
];

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Charset;

impl CharsetTrait for Charset {
    type Alias = Alias;
    type Character = Character;
    type DecodeError = DecodeError;
    type Str = Str;
    type String = String;

    const MIB_ENUM: u16 = 5;
    const PREFERRED_MIME_NAME: Option<&'static UsAsciiStr> =
        Some(unsafe { UsAsciiStr::from_bytes_unchecked(b"ISO-8859-2") });
    const PRIMARY_NAME: &'static UsAsciiStr =
        unsafe { UsAsciiStr::from_bytes_unchecked(b"ISO_8859-2:1987") };
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
        if value < 0xa0 {
            Character(value as char)
        } else {
            Character(GRAPHICS_RIGHT_TO_UNICODE_MAP[(value - 0xa0) as usize])
        }
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
        // Unsafe justification: ISO_8859-2:1987 is an 1-byte charset with a 1-to-1 mapping for each
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
        // Unsafe justification: ISO_8859-2:1987 is an 1-byte charset with a 1-to-1 mapping for each
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
        formatter.write_str("invalid ISO-8859-2:1987 (ISO-8859-2)")
    }
}

impl Error for DecodeError {}

aliases! {
    Alias,

    (CsIsoLatin2, b"csISOLatin2");
    (Iso8859_2, b"ISO-8859-2");
    (Iso8859_2Alt, b"ISO_8859-2");
    (IsoIr101, b"iso-ir-101");
    (L2, b"l2");
    (Latin2, b"latin2");
}
