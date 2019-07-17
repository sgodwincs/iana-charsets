use std::borrow::{Borrow, ToOwned};
use std::error::Error;
use std::fmt::{Debug, Display};
use std::ops::Deref;
use std::hash::Hash;

use crate::charsets::UsAsciiStr;

macro_rules! aliases {
    (
        $enum:ident,
        $(
            ($variant:ident, $value:expr);
        )+
    ) => {
        use crate::charset::Alias as AliasTrait;
        use crate::charsets::UsAsciiStr;

        pub enum $enum {
        $(
            $variant,
        )+
        }

        impl AliasTrait for $enum {
            fn name(&self) -> &'static UsAsciiStr {
                use self::$enum::*;

                match self {
                $(
                    $variant => unsafe { UsAsciiStr::from_bytes_unchecked($value) }
                )+
                }
            }
        }
    };
}

pub trait Alias: Sized {
    fn name(&self) -> &'static UsAsciiStr;
}

pub trait Character: Copy {}

pub trait Str:
    AsRef<[u8]>
    + Debug
    + Display
    + Eq
    + Hash
    + Ord
    + PartialEq
    + PartialOrd
    + private::Sealed
    + ToOwned<Owned = <Self as Str>::String>
where
    Vec<u8>: From<Self::String>
{
    type DecodeError: Error;
    type String: String<DecodeError = Self::DecodeError, Str = Self>;

    fn decode(value: &[u8]) -> Result<&Self, Self::DecodeError>;
    unsafe fn decode_unchecked(value: &[u8]) -> &Self;
}

pub trait String:
    AsRef<[u8]>
    + AsRef<<Self as String>::Str>
    + Borrow<<Self as String>::Str>
    + Clone
    + Debug
    + Deref<Target = <Self as String>::Str>
    + Display
    + Eq
    + Hash
    + Ord
    + PartialEq
    + PartialOrd
    + private::Sealed
    + Sized
where
    Vec<u8>: From<Self>,
{
    type DecodeError: Error;
    type Str: Str<DecodeError = Self::DecodeError, String = Self> + ?Sized;

    fn decode(value: Vec<u8>) -> Result<Self, (Vec<u8>, Self::DecodeError)>;
    unsafe fn decode_unchecked(value: Vec<u8>) -> Self;
}

pub trait Charset: private::Sealed
where
    Vec<u8>: From<Self::String>
{
    type Alias: Alias;
    type Character: Character;
    type DecodeError: Error;
    type Str: Str<DecodeError = Self::DecodeError, String = Self::String> + ?Sized;
    type String: String<DecodeError = Self::DecodeError, Str = Self::Str>;

    const MIB_ENUM: u16;
    const PREFERRED_MIME_NAME: Option<&'static UsAsciiStr>;
    const PRIMARY_NAME: &'static UsAsciiStr;

    fn is_mime_text_suitable() -> bool {
        Self::PREFERRED_MIME_NAME.is_some()
    }
}

pub(crate) mod private {
    pub trait Sealed {}
}
