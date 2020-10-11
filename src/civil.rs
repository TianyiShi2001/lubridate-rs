pub mod weekday;
use crate::utils::*;
use crate::DateTime;
use std::convert::From;
use weekday::*;

#[derive(PartialOrd, Ord, Eq, PartialEq)]
pub struct CivilDateTime {
    pub(crate) year: u16,
    pub(crate) month: u8,
    pub(crate) day: u8,
    pub(crate) hour: u8,
    pub(crate) minute: u8,
    pub(crate) second: u8,
    // pub(crate) nanos: u32
}

impl CivilDateTime {
    pub fn new(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Self {
        let is_leap = is_leap_year(year);
        assert!(0 < month && month <= 12, "month must be between 1 and 12!");
        // let mdays = MDAYS[month as usize];
        assert!(
            check_mdays(month, day, is_leap),
            "The month does not contain that many days!"
        );
        assert!(hour < 24, "hour must be between 0 and 23!");
        assert!(minute < 60, "minute must be between 0 and 60!");
        assert!(second < 60, "second must be between 0 and 60!"); // TODO: allow leap secs?
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }
}

impl CivilDateTime {
    pub fn wday(&self) -> Weekday {
        let mut wd = 2400 + self.year % 400 + if self.month < 3 { 1 } else { 0 };
        wd += wd / 4 - wd / 100 + wd / 400;
        wd += (WEEKDAY_OFFSETS_M[self.month as usize] + self.day as i8) as u16;
        WEEKDAY_OFFSETS[wd as usize % 7 + 6]
    }
    pub fn yday(&self) -> u16 {
        const OFFSET: [u16; 1 + 12] = [0, 0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
        let feb29 = self.month > 2 && is_leap_year(self.year);
        OFFSET[self.month as usize] + self.day as u16 + if feb29 { 1 } else { 0 }
    }
}

// impl From<DateTime> for CivilTime {

// }

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;

    #[test]
    fn test_wday_yday() {
        let dttm = CivilDateTime::new(2020, 10, 11, 12, 28, 20);
        assert_eq!(dttm.wday(), crate::civil::weekday::Weekday::Sun); // 1602419300
        assert_eq!(dttm.yday(), 285); // 1602419300
    }

    #[test]
    fn test_comparison() {
        let dttm1 = CivilDateTime::new(2020, 10, 11, 12, 28, 20);
        let dttm2 = CivilDateTime::new(2020, 10, 12, 01, 01, 01);
        let dttm3 = CivilDateTime::new(2020, 09, 11, 23, 59, 59);
        assert!(dttm2 > dttm1);
        assert!(dttm3 < dttm1);
        assert!(dttm1 == dttm1);
    }
}
