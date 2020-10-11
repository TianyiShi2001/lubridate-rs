#![rustfmt::skip]

pub(crate) const SECONDS_IN_ONE_SECOND: u32 =        1;
pub(crate) const SECONDS_IN_ONE_MINUTE: u32 =       60;
pub(crate) const SECONDS_IN_ONE_HOUR  : u32 =     3600;
pub(crate) const SECONDS_IN_ONE_DAY   : u32 =    86400;
pub(crate) const SECONDS_IN_ONE_WEEK  : u32 =   604800;
pub(crate) const SECONDS_IN_ONE_YEAR  : u32 = 31557600; // 365.25 days!

pub(crate) const SECONDS_IN_ONE: [u32; 6] = [
    SECONDS_IN_ONE_SECOND,
    SECONDS_IN_ONE_MINUTE,
    SECONDS_IN_ONE_HOUR,
    SECONDS_IN_ONE_DAY,
    SECONDS_IN_ONE_WEEK,
    SECONDS_IN_ONE_YEAR
];

/// start of each month in seconds in a common year (1 indexed)
pub(crate) const SM: [u32; 14] = [ 0,
           0,
     2678400, 
     5097600, 
     7776000, 
    10368000,
    13046400,
    15638400,
    18316800,
    20995200,
    23587200,
    26265600,
    28857600,
    31536000,
];

/// days in months
pub(crate) const MDAYS: [u8; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
/// seconds in a day: 24*60*60
pub(crate) const DAYLEN: u32 = 86400;
/// seconds between 2000-01-01 and 1970-01-01
pub(crate) const D30: u32 = 946684800;
/// common year in sec: 365*24*60*60
pub(crate) const YEARLEN: u32 = 31536000;