use paste::item;
use std::borrow::ToOwned;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub mod iso8859_1_1987;
pub mod iso8859_2_1987;
pub mod iso8859_3_1988;
pub mod us_ascii;
pub mod utf_8;

pub use iso8859_1_1987::{
    Alias as Iso8859_1_1987Alias, Character as Iso8859_1_1987Character,
    Charset as Iso8859_1_1987Charset, DecodeError as Iso8859_1_1987DecodeError,
    Str as Iso8859_1_1987Str, String as Iso8859_1_1987String,
};
pub use iso8859_2_1987::{
    Alias as Iso8859_2_1987Alias, Character as Iso8859_2_1987Character,
    Charset as Iso8859_2_1987Charset, DecodeError as Iso8859_2_1987DecodeError,
    Str as Iso8859_2_1987Str, String as Iso8859_2_1987String,
};
pub use iso8859_3_1988::{
    Alias as Iso8859_3_1988Alias, Character as Iso8859_3_1988Character,
    Charset as Iso8859_3_1988Charset, DecodeError as Iso8859_3_1988DecodeError,
    Str as Iso8859_3_1988Str, String as Iso8859_3_1988String,
};
pub use us_ascii::{
    Alias as UsAsciiAlias, Character as UsAsciiCharacter, Charset as UsAsciiCharset,
    DecodeError as UsAsciiDecodeError, Str as UsAsciiStr, String as UsAsciiString,
};
pub use utf_8::{
    Alias as Utf8Alias, Character as Utf8Character, Charset as Utf8Charset,
    DecodeError as Utf8DecodeError, Str as Utf8Str, String as Utf8String,
};

use crate::charset::{Str as StrTrait, String as StringTrait};

macro_rules! enums {
    ($($charset:ident,)+) => {
        pub enum Charset {
        $(
            $charset,
        )+
        }

        impl Charset {
            item! {
                pub fn decode_from_byte_slice<'str>(
                    &self,
                    value: &'str [u8]
                ) -> Result<Str<'str>, DecodeError> {
                    use self::Charset::*;

                    match self {
                    $(
                        $charset => Ok(Str::$charset([<$charset Str>]::decode(value)?)),
                    )+
                    }
                }
            }

            item! {
                pub fn decode_from_byte_vec(
                    &self,
                    value: Vec<u8>
                ) -> Result<String, (Vec<u8>, DecodeError)> {
                    use self::Charset::*;

                    match self {
                    $(
                        $charset => Ok(String::$charset(
                            [<$charset String>]::decode(value)
                                .map_err(|(value, error)| (value, DecodeError::from(error)))?)),
                    )+
                    }
                }
            }
        }

        item! {
            #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
            pub enum Character {
            $(
                $charset([<$charset Character>]),
            )+
            }
        }

        impl Display for Character {
            fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
                use self::Character::*;

                match self {
                $(
                    $charset(character) => character.fmt(formatter),
                )+
                }
            }
        }

        item! {
            /// We cannot implement [`std::borrow::ToOwned`] as there is no way to implement
            /// [`std::borrow::Borrow`] for [`String`] as we would have to return a reference to a
            /// newly constructed [`Str`].
            ///
            /// This can be changed if DST enums are ever
            /// supported (see https://github.com/rust-lang/rfcs/issues/1151).
            ///
            /// The ideal implementation here is something like:
            ///
            /// ```ignore
            /// pub enum Str {
            ///     // ...
            ///
            ///     UsAscii(UsAsciiStr),
            ///     Utf8(Utf8Str),
            ///
            ///     // ...
            /// }
            /// ```
            ///
            /// which can be used like:
            ///
            /// ```ignore
            /// let a = &Str::UsAscii(us_ascii_str);
            /// ```
            ///
            /// The workaround is to instead use the [`From`] implementation to construct a
            /// [`String`] from a [`Str`].
            #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
            pub enum Str<'str> {
            $(
                $charset(&'str [<$charset Str>]),
            )+
            }
        }

        impl Str<'_> {
            pub fn to_owned(&self) -> String {
                use self::Str::*;

                match *self {
                $(
                    $charset(str) => String::$charset(str.to_owned()),
                )+
                }
            }
        }

        impl AsRef<[u8]> for Str<'_> {
            fn as_ref(&self) -> &[u8] {
                use self::Str::*;

                match self {
                $(
                    $charset(str) => str.as_ref(),
                )+
                }
            }
        }

        impl Display for Str<'_> {
            fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
                use self::Str::*;

                match self {
                $(
                    $charset(str) => str.fmt(formatter),
                )+
                }
            }
        }

        item! {
            /// We cannot implement [`std::convert::AsRef`], [`std::borrow::Borrow`], or
            /// [`std::ops::Deref`] as we cannot return a reference to the constructed [`Str`] on
            /// the stack.
            #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
            pub enum String {
            $(
                $charset([<$charset String>]),
            )+
            }
        }

        impl String {
            pub fn as_ref(&self) -> Str {
                self.deref()
            }

            pub fn borrow(&self) -> Str {
                self.deref()
            }

            pub fn deref(&self) -> Str {
                use self::String::*;

                match self {
                $(
                    $charset(string) => Str::$charset(&string),
                )+
                }
            }
        }

        impl AsRef<[u8]> for String {
            fn as_ref(&self) -> &[u8] {
                use self::String::*;

                match self {
                $(
                    $charset(string) => string.as_ref(),
                )+
                }
            }
        }

        impl Display for String {
            fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
                use self::String::*;

                match self {
                $(
                    $charset(string) => string.fmt(formatter),
                )+
                }
            }
        }

        impl<'str> From<Str<'str>> for String {
            fn from(value: Str<'str>) -> Self {
                value.to_owned()
            }
        }

        impl From<String> for Vec<u8> {
            fn from(value: String) -> Self {
                use self::String::*;

                match value {
                $(
                    $charset(string) => string.into(),
                )+
                }
            }
        }

        item! {
            #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
            pub enum DecodeError {
            $(
                $charset([<$charset DecodeError>]),
            )+
            }
        }

        impl Display for DecodeError {
            fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
                use self::DecodeError::*;

                match self {
                $(
                    $charset(error) => error.fmt(formatter),
                )+
                }
            }
        }

        impl Error for DecodeError {
            fn cause(&self) -> Option<&dyn Error> {
                use self::DecodeError::*;

                match self {
                $(
                    $charset(error) => Some(error),
                )+
                }
            }
        }

    $(
        item! {
            impl From<[<$charset DecodeError>]> for DecodeError {
                fn from(value: [<$charset DecodeError>]) -> Self {
                    DecodeError::$charset(value)
                }
            }
        }
    )+
    };
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CowStr<'str> {
    Borrowed(Str<'str>),
    Owned(String),
}

impl CowStr<'_> {
    pub fn into_owned(self) -> String {
        use self::CowStr::*;

        match self {
            Borrowed(str) => str.into(),
            Owned(string) => string,
        }
    }

    pub fn to_mut(&mut self) -> &mut String {
        use self::CowStr::*;

        match *self {
            Borrowed(str) => {
                *self = Owned(str.into());

                match *self {
                    Borrowed(_) => unreachable!(),
                    Owned(ref mut string) => string,
                }
            }
            Owned(ref mut string) => string,
        }
    }
}

impl Display for CowStr<'_> {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        use self::CowStr::*;

        match self {
            Borrowed(str) => str.fmt(formatter),
            Owned(string) => string.fmt(formatter),
        }
    }
}

impl<'str> From<Str<'str>> for CowStr<'str> {
    fn from(value: Str<'str>) -> Self {
        CowStr::Borrowed(value)
    }
}

impl From<String> for CowStr<'static> {
    fn from(value: String) -> Self {
        CowStr::Owned(value)
    }
}

enums! {
    Iso8859_1_1987,
    Iso8859_2_1987,
    Iso8859_3_1988,
    UsAscii,
    Utf8,
}
