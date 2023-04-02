// This is a part of Chrono.
// See README.md and LICENSE.txt for details.

//! The local (system) time zone.

#[cfg(feature = "rkyv")]
use rkyv::{Archive, Deserialize, Serialize};

use super::fixed::FixedOffset;
use crate::naive::{NaiveDate, NaiveDateTime};
use crate::offset::LocalResult;
#[allow(deprecated)]
use crate::Date;
use crate::{DateTime, Error, TimeZone};

// we don't want `stub.rs` when the target_os is not wasi or emscripten
// as we use js-sys to get the date instead
#[cfg(all(
    not(unix),
    not(windows),
    not(all(
        target_arch = "wasm32",
        feature = "wasmbind",
        not(any(target_os = "emscripten", target_os = "wasi"))
    ))
))]
#[path = "stub.rs"]
mod inner;

#[cfg(unix)]
#[path = "unix.rs"]
mod inner;

#[cfg(windows)]
#[path = "windows.rs"]
mod inner;

#[cfg(unix)]
mod tz_info;

/// The local timescale. This is implemented via the standard `time` crate.
///
/// Using the [`TimeZone`](./trait.TimeZone.html) methods
/// on the Local struct is the preferred way to construct `DateTime<Local>`
/// instances.
///
/// # Example
///
/// ```
/// use chrono::{Local, DateTime, TimeZone};
///
/// let dt: DateTime<Local> = Local::now()?;
/// let dt: DateTime<Local> = Local.timestamp(0, 0)?;
/// # Ok::<_, chrono::Error>(())
/// ```
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "rkyv", derive(Archive, Deserialize, Serialize))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Local;

impl Local {
    /// Returns a `Date` which corresponds to the current date.
    #[allow(deprecated)]
    pub fn today() -> Result<Date<Local>, Error> {
        Ok(Local::now()?.date())
    }

    /// Returns a `DateTime` which corresponds to the current date and time.
    #[cfg(not(all(
        target_arch = "wasm32",
        feature = "wasmbind",
        not(any(target_os = "emscripten", target_os = "wasi"))
    )))]
    pub fn now() -> Result<DateTime<Local>, Error> {
        inner::now()
    }

    /// Returns a `DateTime` which corresponds to the current date and time.
    #[cfg(all(
        target_arch = "wasm32",
        feature = "wasmbind",
        not(any(target_os = "emscripten", target_os = "wasi"))
    ))]
    pub fn now() -> Result<DateTime<Local>, Error> {
        use super::Utc;
        let now: DateTime<Utc> = super::Utc::now()?;

        // Workaround missing timezone logic in `time` crate
        let offset = FixedOffset::west((js_sys::Date::new_0().get_timezone_offset() as i32) * 60)?;
        Ok(DateTime::from_utc(now.naive_utc(), offset))
    }
}

impl TimeZone for Local {
    type Offset = FixedOffset;

    fn from_offset(_offset: &FixedOffset) -> Local {
        Local
    }

    fn offset_from_local_datetime(&self, local: &NaiveDateTime) -> Result<LocalResult<FixedOffset>, Error> {
        self.from_local_datetime(local)?.single()?.offset()
    }

    fn offset_from_utc_datetime(&self, utc: &NaiveDateTime) -> FixedOffset {
        *self.from_utc_datetime(utc).offset()
    }

    #[cfg(all(
        target_arch = "wasm32",
        feature = "wasmbind",
        not(any(target_os = "emscripten", target_os = "wasi"))
    ))]
    fn from_local_datetime(&self, local: &NaiveDateTime) -> Result<LocalResult<DateTime<Local>>, Error> {
        let mut local = local.clone();
        // Get the offset from the js runtime
        let offset =
            FixedOffset::west((js_sys::Date::new_0().get_timezone_offset() as i32) * 60)?;
        local -= crate::TimeDelta::seconds(offset.local_minus_utc() as i64);
        Ok(LocalResult::Single(DateTime::from_utc(local, offset)))
    }

    #[cfg(not(all(
        target_arch = "wasm32",
        feature = "wasmbind",
        not(any(target_os = "emscripten", target_os = "wasi"))
    )))]
    fn from_local_datetime(&self, local: &NaiveDateTime) -> Result<LocalResult<DateTime<Local>>, Error> {
        inner::naive_to_local(local, true)
    }

    #[cfg(all(
        target_arch = "wasm32",
        feature = "wasmbind",
        not(any(target_os = "emscripten", target_os = "wasi"))
    ))]
    fn from_utc_datetime(&self, utc: &NaiveDateTime) -> DateTime<Local> {
        // Get the offset from the js runtime
        let offset =
            FixedOffset::west_opt((js_sys::Date::new_0().get_timezone_offset() as i32) * 60)
                .unwrap();
        DateTime::from_utc(*utc, offset)
    }

    #[cfg(not(all(
        target_arch = "wasm32",
        feature = "wasmbind",
        not(any(target_os = "emscripten", target_os = "wasi"))
    )))]
    fn from_utc_datetime(&self, utc: &NaiveDateTime) -> DateTime<Local> {
        // this is OK to unwrap as getting local time from a UTC
        // timestamp is never ambiguous
        inner::naive_to_local(utc, false).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Local;
    use crate::offset::TimeZone;
    use crate::{Datelike, Error, TimeDelta};
    #[cfg(unix)]
    use crate::{NaiveDate, NaiveDateTime, Timelike};

    #[cfg(unix)]
    use std::{path, process};

    #[cfg(unix)]
    fn verify_against_date_command_local(
        path: &'static str,
        dt: NaiveDateTime,
    ) -> Result<(), Error> {
        let output = process::Command::new(path)
            .arg("-d")
            .arg(format!("{}-{:02}-{:02} {:02}:05:01", dt.year(), dt.month(), dt.day(), dt.hour()))
            .arg("+%Y-%m-%d %H:%M:%S %:z")
            .output()
            .unwrap();

        let date_command_str = String::from_utf8(output.stdout)?;

        match Local.from_local_datetime(
            &NaiveDate::from_ymd(dt.year(), dt.month(), dt.day())?.and_hms(dt.hour(), 5, 1)?,
        ) {
            // compare a legit date to the "date" output
            Ok(crate::LocalResult::Single(dt)) => assert_eq!(format!("{}\n", dt), date_command_str),
            // "date" command always returns a given time when it is ambiguous (dt.earliest())
            Ok(crate::LocalResult::Ambiguous(dt1, _dt2)) => {
                assert_eq!(format!("{}\n", dt1), date_command_str)
            }
            // "date" command returns an empty string for an invalid time (e.g. spring forward gap due to DST)
            Err(_) => assert_eq!(date_command_str, ""),
        }
        Ok(())
    }

    #[test]
    #[cfg(unix)]
    fn try_verify_against_date_command() -> Result<(), Error> {
        let date_path = "/usr/bin/date";

        if !path::Path::new(date_path).exists() {
            // date command not found, skipping
            // avoid running this on macOS, which has path /bin/date
            // as the required CLI arguments are not present in the
            // macOS build.
            return Ok(());
        }

        let mut date = NaiveDate::from_ymd(1975, 1, 1).unwrap().and_hms(0, 0, 0).unwrap();

        while date.year() < 2078 {
            if (1975..=1977).contains(&date.year())
                || (2020..=2022).contains(&date.year())
                || (2073..=2077).contains(&date.year())
            {
                verify_against_date_command_local(date_path, date)?;
            }

            date += crate::TimeDelta::hours(1);
        }
        Ok(())
    }

    #[test]
    fn verify_correct_offsets() {
        let now = Local::now().unwrap();
        let from_local = Local.from_local_datetime(&now.naive_local()).unwrap().unwrap();
        let from_utc = Local.from_utc_datetime(&now.naive_utc()).unwrap();

        assert_eq!(now.offset().local_minus_utc(), from_local.offset().local_minus_utc());
        assert_eq!(now.offset().local_minus_utc(), from_utc.offset().local_minus_utc());

        assert_eq!(now, from_local);
        assert_eq!(now, from_utc);
    }

    #[test]
    fn verify_correct_offsets_distant_past() {
        // let distant_past = Local::now() - Duration::days(365 * 100);
        let distant_past = Local::now().unwrap() - TimeDelta::days(250 * 31);
        let from_local = Local.from_local_datetime(&distant_past.naive_local()).unwrap().unwrap();
        let from_utc = Local.from_utc_datetime(&distant_past.naive_utc()).unwrap();

        assert_eq!(distant_past.offset().local_minus_utc(), from_local.offset().local_minus_utc());
        assert_eq!(distant_past.offset().local_minus_utc(), from_utc.offset().local_minus_utc());

        assert_eq!(distant_past, from_local);
        assert_eq!(distant_past, from_utc);
    }

    #[test]
    fn verify_correct_offsets_distant_future() {
        let distant_future = Local::now().unwrap() + TimeDelta::days(250 * 31);
        let from_local = Local.from_local_datetime(&distant_future.naive_local()).unwrap().unwrap();
        let from_utc = Local.from_utc_datetime(&distant_future.naive_utc()).unwrap();

        assert_eq!(
            distant_future.offset().local_minus_utc(),
            from_local.offset().local_minus_utc()
        );
        assert_eq!(distant_future.offset().local_minus_utc(), from_utc.offset().local_minus_utc());

        assert_eq!(distant_future, from_local);
        assert_eq!(distant_future, from_utc);
    }

    #[test]
    fn test_local_date_sanity_check() {
        // issue #27
        assert_eq!(Local.ymd(2999, 12, 28).unwrap().unwrap().day(), 28);
    }

    #[test]
    fn test_leap_second() {
        // issue #123
        let today = Local::today().unwrap();

        let dt = today.and_hms_milli(1, 2, 59, 1000).unwrap();
        let timestr = dt.time().to_string();
        // the OS API may or may not support the leap second,
        // but there are only two sensible options.
        assert!(timestr == "01:02:60" || timestr == "01:03:00", "unexpected timestr {:?}", timestr);

        let dt = today.and_hms_milli(1, 2, 3, 1234).unwrap();
        let timestr = dt.time().to_string();
        assert!(
            timestr == "01:02:03.234" || timestr == "01:02:04.234",
            "unexpected timestr {:?}",
            timestr
        );
    }
}
