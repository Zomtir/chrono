//! Error type
use core::fmt;

/// Error type for date and time operations.
// TODO: Error sources that are not yet covered are the platform APIs, the parsing of a `TZfile` and
// parsing of a `TZ` environment variable.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// One or more of the arguments to a function are invalid.
    ///
    /// An example is creating a `NaiveTime` with 25 as the hour value.
    InvalidParameter,

    /// The result, or an intermediate value necessary for calculating a result, would be out of
    /// range.
    ///
    /// An example is a date for the year 500.000, which is out of the range supported by chrono's
    /// types.
    OutOfRange,

    /// A date or datetime does not exist.
    ///
    /// Examples are:
    /// - April 31,
    /// - February 29 in a non-leap year,
    /// - a time that falls in the gap created by moving the clock forward during a DST transition,
    /// - a leap second on a non-minute boundary.
    DoesNotExist,

    /// Some of the date or time components are not consistent with each other.
    ///
    /// An example is parsing 'Sunday 2023-04-21', while that date is a Friday.
    Inconsistent,

    /// Character does not match with the expected format.
    ///
    /// Contains the byte index of the character where the input diverges.
    InvalidCharacter(u32),

    /// Value is not allowed by the format (during parsing).
    ///
    /// Examples are a number that is larger or smaller than the defined range, or the name of a
    /// weekday, month or timezone that doesn't match.
    ///
    /// Contains the byte index pointing at the start of the invalid value.
    InvalidValue(u32),

    /// The format string contains a formatting specifier that is not supported.
    ///
    /// Contains the byte index of the formatting specifier within the format string.
    UnsupportedSpecifier(u32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidParameter => write!(f, "invalid parameter"),
            Error::OutOfRange => write!(f, "date outside of the supported range"),
            Error::DoesNotExist => write!(f, "date or datetime does not exist"),
            Error::Inconsistent => {
                write!(f, "some of the date or time components are not consistent with each other")
            }
            Error::InvalidCharacter(i) => {
                write!(f, "input doesn't match with the expected format at position {}", i)
            }
            Error::InvalidValue(i) => {
                write!(f, "input has a value not allowed by the format at position {}", i)
            }
            Error::UnsupportedSpecifier(_) => {
                write!(f, "format string contains a formatting specifier that is not supported")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
