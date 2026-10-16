use chrono::NaiveDateTime;

use crate::record::{LogLevel, LogRecord};

pub(crate) fn parse_line(line: &str) -> LogRecord {
    let mut parts = line.split_whitespace();

    let date = parts.next().unwrap();
    let time = parts.next().unwrap();

    let timestamp = NaiveDateTime::parse_from_str(
        &format!("{date} {time}"),
        "%Y-%m-%d %H:%M:%S",
    )
    .unwrap();

    let level = LogLevel::parse(parts.next().unwrap());

    let logger = parts
        .next()
        .unwrap()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .to_owned();

    let message = parts.collect::<Vec<_>>().join(" ");

    LogRecord {
        timestamp,
        level,
        logger,
        message,
    }
}
