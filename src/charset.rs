use std::borrow::{Borrow, ToOwned};
use std::error::Error;

use crate::charsets::UsAsciiStr;

macro_rules! aliases {
    (
        $enum:ident,
        $(
            ($variant:ident, $value:expr);
        )+
    ) => {
        use crate::charset::Alias as AliasTrait;
        use crate::charsets::{UsAsciiStr, UsAsciiCharset};

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
                    $variant => unsafe { UsAsciiCharset::from_bytes_unchecked($value) }
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

pub trait Str<TString>: ToOwned<Owned = TString>
where
    TString: Borrow<Self>,
{
}

pub trait String<TStr>: Borrow<TStr> + Clone + Sized
where
    TStr: Str<Self> + ?Sized,
{
}

pub trait Charset {
    type Alias: Alias;
    type Character: Character;
    type DecodeError: Error;
    type Str: Str<Self::String> + ?Sized;
    type String: String<Self::Str>;

    const MIB_ENUM: u16;
    const PREFERRED_MIME_NAME: Option<&'static UsAsciiStr>;
    const PRIMARY_NAME: &'static UsAsciiStr;

    fn is_mime_text_suitable() -> bool {
        Self::PREFERRED_MIME_NAME.is_some()
    }

    fn decode(value: &[u8]) -> Result<&Self::Str, Self::DecodeError> {
        Self::validate(value)?;
        Ok(unsafe { Self::decode_unchecked(value) })
    }

    unsafe fn decode_unchecked(value: &[u8]) -> &Self::Str;
    fn validate(value: &[u8]) -> Result<(), Self::DecodeError>;
}
