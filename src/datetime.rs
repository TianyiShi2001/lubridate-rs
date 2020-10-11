pub use crate::civil::weekday::*;
use crate::civil::CivilDateTime;
use crate::constants::*;
use crate::constants::{D30, MDAYS, SECONDS_IN_ONE_YEAR};
use crate::utils::{adjust_leap_years, check_mdays, is_leap_year};
use std::convert::From;

pub struct DateTime {
    // pub year: u16,
    // pub month: u8,
    // pub day: u8,
    // pub
    secs: u32,
    nanos: u32,
}

impl DateTime {
    pub fn new(secs: u32, nanos: u32) -> Self {
        Self { secs, nanos }
    }

    pub fn timestamp(&self) -> u32 {
        self.secs
    }

    pub fn from_timestamp(timestamp: u32) -> Self {
        Self {
            secs: timestamp,
            nanos: 0,
        }
    }

    pub fn from_timestamp_f(timestamp: f64) -> Self {
        Self {
            secs: timestamp as u32,
            nanos: ((timestamp - (timestamp as u32 as f64)) * 1e9) as u32,
        }
    }

    pub fn from_ymd_hms(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Self {
        CivilDateTime::new(year, month, day, hour, minute, second).into()
    }
    pub fn from_ymd_hm(year: u16, month: u8, day: u8, hour: u8, minute: u8) -> Self {
        Self::from_ymd_hms(year, month, day, hour, minute, 0)
    }
    pub fn from_ymd_h(year: u16, month: u8, day: u8, hour: u8) -> Self {
        Self::from_ymd_hm(year, month, day, hour, 0)
    }
    pub fn from_ymd(year: u16, month: u8, day: u8) -> Self {
        Self::from_ymd_h(year, month, day, 0)
    }
    pub fn with_nanos(self, nanos: u32) -> Self {
        Self {
            secs: self.secs,
            nanos: nanos,
        }
    }
}

pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl From<CivilDateTime> for DateTime {
    fn from(dttm: CivilDateTime) -> Self {
        let CivilDateTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
        } = dttm;
        let mut secs: u32 = 0;
        secs += second as u32;
        secs += minute as u32 * 60;
        secs += hour as u32 * 3600;
        secs += (day - 1) as u32 * DAYLEN;
        secs += SM[month as usize];
        secs += D30;
        let y: i64 = year as i64 - 2000;
        secs = (secs as i64
            + YEARLEN as i64 * y
            + adjust_leap_years(y, month as i64, is_leap_year(year))) as u32;
        Self { secs, nanos: 0 }
    }
}

impl From<chrono::DateTime<chrono::Utc>> for DateTime {
    fn from(dttm: chrono::DateTime<chrono::Utc>) -> Self {
        Self::from_timestamp(dttm.timestamp() as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;

    #[test]
    fn test_from_chrono_utc() {
        let dttm = chrono::Utc.ymd(2020, 10, 11).and_hms(12, 28, 20);
        let dttm: crate::DateTime = dttm.into();
        let dttm1 = crate::DateTime::from_ymd_hms(2020, 10, 11, 12, 28, 20);
        assert_eq!(dttm.timestamp(), dttm1.timestamp()); // 1602419300
    }
}
