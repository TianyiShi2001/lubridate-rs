# toki

Make working with dates in Rust just that little bit easier 

## Why?

I was spoiled by the [**lubridate**](https://github.com/tidyverse/lubridate) package in the R world and found it can be pain to work with datetimes in Rust, especially when it comes to arithmatics and durations.

**toki** aims to makes datetime arithmatics simple and reliable. It does this by introducing three new time span classes borrowed from [**lubridate**](https://github.com/tidyverse/lubridate), which are, well, borrowed from [**joda**](http://joda.org).

- `durations`, which measure the exact amount of time between two points
- `periods`, which accurately track clock times despite leap years, leap seconds, and day light savings time
- `intervals`, a protean summary of the time information between two points

## About the name

**toki** is the word for "time" in Japanese.