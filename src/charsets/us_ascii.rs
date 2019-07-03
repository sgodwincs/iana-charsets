use std::borrow::{Borrow, ToOwned};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::mem;

use crate::charset::{
    Character as CharacterTrait, Charset as CharsetTrait, Str as StrTrait, String as StringTrait,
};

#[derive(Debug)]
pub struct Charset;

impl Charset {
    pub const unsafe fn from_bytes_unchecked(value: &[u8]) -> &<Self as CharsetTrait>::Str {
        mem::transmute(value)
    }
}

impl CharsetTrait for Charset {
    type Alias = Alias;
    type Character = Character;
    type DecodeError = DecodeError;
    type Str = Str;
    type String = String;

    const MIB_ENUM: u16 = 1;
    const MIME_NAME: Option<&'static Str> =
        Some(unsafe { Self::from_bytes_unchecked(b"US-ASCII") });
    const PRIMARY_NAME: &'static Str = unsafe { Self::from_bytes_unchecked(b"US-ASCII") };

    unsafe fn decode_unchecked(value: &[u8]) -> &Self::Str {
        Self::from_bytes_unchecked(value)
    }

    fn validate(value: &[u8]) -> Result<(), DecodeError> {
        for byte in value {
            if !byte.is_ascii() {
                return Err(DecodeError);
            }
        }

        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct Character(u8);

impl CharacterTrait for Character {}

pub struct Str([<Charset as CharsetTrait>::Character]);

impl StrTrait<String> for Str {}

impl ToOwned for Str {
    type Owned = String;

    fn to_owned(&self) -> Self::Owned {
        String(self.0.to_vec())
    }
}

#[derive(Clone)]
pub struct String(Vec<<Charset as CharsetTrait>::Character>);

impl Borrow<Str> for String {
    fn borrow(&self) -> &Str {
        unsafe { mem::transmute(&*self.0) }
    }
}

impl StringTrait<Str> for String {}

#[derive(Debug)]
pub struct DecodeError;

impl Display for DecodeError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "invalid US-ASCII")
    }
}

impl Error for DecodeError {}

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
