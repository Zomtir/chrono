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
    DateOutOfRange,

    /// A date or datetime does is invalid.
    ///
    /// Examples are:
    /// - April 31,
    /// - February 29 in a non-leap year,
    /// - a time that falls in the gap created by moving the clock forward during a DST transition,
    /// - a leap second on a non-minute boundary.
    InvalidDate,

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

    /// Given field is out of permitted range.
    FieldOutOfRange,

    /// There is no possible date and time value with given set of fields.
    ///
    /// This does not include the out-of-range conditions, which are trivially invalid.
    /// It includes the case that there are one or more fields that are inconsistent to each other.
    FieldImpossible,

    /// Given set of fields is not enough to make a requested date and time value.
    ///
    /// Note that there *may* be a case that given fields constrain the possible values so much
    /// that there is a unique possible value. Chrono only tries to be correct for
    /// most useful sets of fields however, as such constraint solving can be expensive.
    FieldNotEnough,

    /// The input string has some invalid character sequence for given formatting items.
    InvalidInput,

    /// The input string has been prematurely ended.
    InputTooShort,

    /// All formatting items have been read but there is a remaining input.
    InputTooLong,

    /// There was an error on the formatting string, or there were non-supported formating items.
    BadFormat,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidParameter => write!(f, "invalid parameter"),
            Error::DateOutOfRange => write!(f, "date outside of the supported range"),
            Error::InvalidDate => write!(f, "date or datetime does not exist"),
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
            Error::FieldOutOfRange => write!(f, "input is out of range"),
            Error::FieldImpossible => write!(f, "no possible date and time matching input"),
            Error::FieldNotEnough => write!(f, "input is not enough for unique date and time"),
            Error::InvalidInput => write!(f, "input contains invalid characters"),
            Error::InputTooShort => write!(f, "premature end of input"),
            Error::InputTooLong => write!(f, "trailing input"),
            Error::BadFormat => write!(f, "bad or unsupported format string"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

pub(crate) const INVALID_PARAM: Error = Error::InvalidParameter;
pub(crate) const INVALID_DATE: Error = Error::InvalidDate;

pub(crate) const FIELD_OUT_OF_RANGE: Error = Error::FieldOutOfRange;
pub(crate) const FIELD_IMPOSSIBLE: Error = Error::FieldImpossible;
pub(crate) const FIELD_NOT_ENOUGH: Error = Error::FieldNotEnough;
pub(crate) const INVALID_INPUT: Error = Error::InvalidInput;
pub(crate) const INPUT_TOO_SHORT: Error = Error::InputTooShort;
pub(crate) const INPUT_TOO_LONG: Error = Error::InputTooLong;
pub(crate) const BAD_FORMAT: Error = Error::BadFormat;