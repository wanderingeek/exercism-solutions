#![warn(clippy::all, clippy::pedantic)]
use time::{Date, Duration, PrimitiveDateTime as DateTime, Time};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    //todo!("What time is a gigasecond later than {start}");

    let one_gigasecond = Duration::new(1_000_000_000, 0);

    start + one_gigasecond
}