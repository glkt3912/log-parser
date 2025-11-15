use crate::core::{LogEntry, Result, LogParserError};
use crate::filters::Filter;
use chrono::{DateTime, Utc, NaiveDate, NaiveTime};

pub struct TimeFilter {
    since: Option<DateTime<Utc>>,
    until: Option<DateTime<Utc>>,
}

impl TimeFilter {
    pub fn new(since: Option<&str>, until: Option<&str>) -> Result<Self> {
        let since_date = if let Some(since_str) = since {
            Some(parse_date_time(since_str)?)
        } else {
            None
        };

        let until_date = if let Some(until_str) = until {
            Some(parse_date_time(until_str)?)
        } else {
            None
        };

        // Validate that since is not after until
        if let (Some(since_dt), Some(until_dt)) = (since_date, until_date) {
            if since_dt > until_dt {
                return Err(LogParserError::Config {
                    message: "since date cannot be after until date".to_string(),
                }.into());
            }
        }

        Ok(Self {
            since: since_date,
            until: until_date,
        })
    }

    pub fn with_since(since: DateTime<Utc>) -> Self {
        Self {
            since: Some(since),
            until: None,
        }
    }

    pub fn with_until(until: DateTime<Utc>) -> Self {
        Self {
            since: None,
            until: Some(until),
        }
    }

    pub fn with_range(since: DateTime<Utc>, until: DateTime<Utc>) -> Result<Self> {
        if since > until {
            return Err(LogParserError::Config {
                message: "since date cannot be after until date".to_string(),
            }.into());
        }

        Ok(Self {
            since: Some(since),
            until: Some(until),
        })
    }
}

impl Filter for TimeFilter {
    fn apply(&self, entry: &LogEntry) -> Result<bool> {
        let entry_timestamp = match &entry.timestamp {
            Some(timestamp) => timestamp,
            None => return Ok(false), // No timestamp means we can't filter by time
        };

        // Check since constraint
        if let Some(since) = &self.since {
            if entry_timestamp < since {
                return Ok(false);
            }
        }

        // Check until constraint
        if let Some(until) = &self.until {
            if entry_timestamp > until {
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn name(&self) -> &'static str {
        "time"
    }
}

// Helper function to parse various date/time formats
fn parse_date_time(date_str: &str) -> Result<DateTime<Utc>> {
    // Try different formats
    let formats = [
        // Full date-time formats
        "%Y-%m-%d %H:%M:%S",      // 2024-01-01 12:00:00
        "%Y-%m-%dT%H:%M:%S",      // 2024-01-01T12:00:00
        "%Y-%m-%d %H:%M:%SZ",     // 2024-01-01 12:00:00Z
        "%Y-%m-%dT%H:%M:%SZ",     // 2024-01-01T12:00:00Z
        "%Y-%m-%d %H:%M",         // 2024-01-01 12:00
        "%Y-%m-%dT%H:%M",         // 2024-01-01T12:00
    ];

    // Try parsing with explicit timezone first
    for format in &formats {
        if let Ok(dt) = DateTime::parse_from_str(date_str, format) {
            return Ok(dt.with_timezone(&Utc));
        }
    }

    // Try parsing as naive datetime and assume UTC
    use chrono::NaiveDateTime;
    for format in &formats {
        if let Ok(naive_dt) = NaiveDateTime::parse_from_str(date_str, format) {
            return Ok(naive_dt.and_utc());
        }
    }

    // Try parsing as date only (assume start/end of day)
    let date_formats = [
        "%Y-%m-%d",               // 2024-01-01
        "%Y/%m/%d",               // 2024/01/01
        "%m/%d/%Y",               // 01/01/2024
        "%d/%m/%Y",               // 01/01/2024 (European format)
    ];

    for format in &date_formats {
        if let Ok(date) = NaiveDate::parse_from_str(date_str, format) {
            // For date-only input, assume start of day (00:00:00)
            let datetime = date.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap());
            return Ok(datetime.and_utc());
        }
    }

    Err(LogParserError::InvalidDateFormat {
        date: date_str.to_string(),
    }.into())
}
