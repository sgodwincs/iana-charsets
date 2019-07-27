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
    '\u{00a0}', '\u{0126}', '\u{02d8}', '\u{00a3}', '\u{00a4}', '\0',       '\u{0124}', '\u{00a7}', '\u{00a8}', '\u{0130}', '\u{015e}', '\u{011e}', '\u{0134}', '\u{00ad}', '\0',       '\u{017b}',
    '\u{00b0}', '\u{0127}', '\u{00b2}', '\u{00b3}', '\u{00b4}', '\u{00b5}', '\u{0125}', '\u{00b7}', '\u{00b8}', '\u{0131}', '\u{015f}', '\u{011f}', '\u{0135}', '\u{00bd}', '\0',       '\u{017c}',
    '\u{00c0}', '\u{00c1}', '\u{00c2}', '\0',       '\u{00c4}', '\u{010a}', '\u{0108}', '\u{00c7}', '\u{00c8}', '\u{00c9}', '\u{00ca}', '\u{00cb}', '\u{00cc}', '\u{00cd}', '\u{00ce}', '\u{00cf}',
    '\0',       '\u{00d1}', '\u{00d2}', '\u{00d3}', '\u{00d4}', '\u{0120}', '\u{00d6}', '\u{00d7}', '\u{011c}', '\u{00d9}', '\u{00da}', '\u{00db}', '\u{00dc}', '\u{016c}', '\u{015c}', '\u{00df}',
    '\u{00e0}', '\u{00e1}', '\u{00e2}', '\0',       '\u{00e4}', '\u{010b}', '\u{0109}', '\u{00e7}', '\u{00e8}', '\u{00e9}', '\u{00ea}', '\u{00eb}', '\u{00ec}', '\u{00ed}', '\u{00ee}', '\u{00ef}',
    '\0',       '\u{00f1}', '\u{00f2}', '\u{00f3}', '\u{00f4}', '\u{0121}', '\u{00f6}', '\u{00f7}', '\u{011d}', '\u{00f9}', '\u{00fa}', '\u{00fb}', '\u{00fc}', '\u{016d}', '\u{015d}', '\u{02d9}',
];

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Charset;

impl CharsetTrait for Charset {
    type Alias = Alias;
    type Character = Character;
    type DecodeError = DecodeError;
    type Str = Str;
    type String = String;

    const MIB_ENUM: u16 = 6;
    const PREFERRED_MIME_NAME: Option<&'static UsAsciiStr> =
        Some(unsafe { UsAsciiStr::from_bytes_unchecked(b"ISO-8859-3") });
    const PRIMARY_NAME: &'static UsAsciiStr =
        unsafe { UsAsciiStr::from_bytes_unchecked(b"ISO_8859-3:1988") };
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
        validate(value)?;
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
        if let Err(error) = validate(&value) {
            return Err((value, error));
        }

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
        formatter.write_str("invalid ISO-8859-3:1988 (ISO-8859-3)")
    }
}

impl Error for DecodeError {}

fn validate(value: &[u8]) -> Result<(), DecodeError> {
    for &byte in value {
        if byte > 0xa0 && GRAPHICS_RIGHT_TO_UNICODE_MAP[(byte - 0xa0) as usize] == '\0' {
            return Err(DecodeError);
        }
    }

    Ok(())
}

aliases! {
    Alias,

    (CsIsoLatin3, b"csISOLatin3");
    (Iso8859_3, b"ISO-8859-3");
    (Iso8859_3Alt, b"ISO_8859-3");
    (IsoIr101, b"iso-ir-109");
    (Latin3, b"latin3");
    (L3, b"l3");
}
