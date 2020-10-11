use crate::constants::*;

#[inline(always)]
pub(crate) fn is_leap_year(y: u16) -> bool {
    ((y) % 4 == 0) && !((y) % 100 == 0 && (y) % 400 != 0)
}

/// return adjustment (in seconds) due to leap years
/// y: years after (positive) or before (negative) 2000-01-01 00:00:00
pub(crate) fn adjust_leap_years(y: i64, m: i64, is_leap: bool) -> i64 {
    let mut secs: i64 = 0;
    if y >= 0 {
        // ordinary leap days after 2000-01-01 00:00:00
        secs += (y / 4 + 1) * DAYLEN as i64;
        if y > 99 {
            secs += (y / 400 - y / 100) * DAYLEN as i64;
        }

        // adjust if within a leap year
        if is_leap && m < 3 {
            secs -= DAYLEN as i64
        }
    } else {
        secs += (y / 4) * DAYLEN as i64;
        if y < -99 {
            secs += (y / 400 - y / 100) * DAYLEN as i64;
        }
        if is_leap && m > 2 {
            secs += DAYLEN as i64;
        }
    }
    secs
}

pub(crate) fn check_mdays(m: u8, d: u8, is_leap: bool) -> bool {
    d <= if m == 2 {
        if is_leap {
            29
        } else {
            28
        }
    } else {
        MDAYS[d as usize]
    }
}
