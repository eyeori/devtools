use rmcp::schemars;
use rmcp::schemars::JsonSchema;
use rmcp::serde_json::Number;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TimestampToDatetimeParams {
    /// The timestamp to convert
    pub timestamp: Number,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DatetimeToTimestampParams {
    /// The year of the datetime, should later than 1970
    pub year: Number,
    /// The month of the datetime, can be 1-12
    pub month: Number,
    /// The day of the datetime, can be 1-31
    pub day: Number,
    /// The hour of the datetime, can be 0-23
    pub hour: Number,
    /// The minute of the datetime, can be 0-59
    pub minute: Number,
    /// The second of the datetime, can be 0-59
    pub second: Number,
    /// The timezone offset of the datetime, like: +08:00, -07:30, default is +08:00
    pub zone_offset: Option<String>,
}

impl TimestampToDatetimeParams {
    pub fn timestamp_millis(&self) -> Option<i64> {
        let mut timestamp = self.timestamp.as_i64().unwrap_or_default();
        let timestamp_len = timestamp.to_string().len();
        if timestamp < 0 || (timestamp_len != 10 && timestamp_len != 13) {
            return None;
        }
        if timestamp_len == 10 {
            timestamp *= 1000;
        }
        Some(timestamp)
    }
}

impl DatetimeToTimestampParams {
    #[inline]
    pub fn year(&self) -> i32 {
        self.year.as_i64().unwrap_or_default() as i32
    }

    #[inline]
    pub fn month(&self) -> u32 {
        self.month.as_u64().unwrap_or_default() as u32
    }

    #[inline]
    pub fn day(&self) -> u32 {
        self.day.as_u64().unwrap_or_default() as u32
    }

    #[inline]
    pub fn hour(&self) -> u32 {
        self.hour.as_u64().unwrap_or_default() as u32
    }

    #[inline]
    pub fn minute(&self) -> u32 {
        self.minute.as_u64().unwrap_or_default() as u32
    }

    #[inline]
    pub fn second(&self) -> u32 {
        self.second.as_u64().unwrap_or_default() as u32
    }

    #[inline]
    pub fn zone_offset(&self) -> &str {
        self.zone_offset.as_deref().unwrap_or("+08:00")
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        self.year() > 0
            && (1..=12).contains(&self.month())
            && (1..=31).contains(&self.day())
            && (0..24).contains(&self.hour())
            && (0..60).contains(&self.minute())
            && (0..60).contains(&self.second())
    }

    #[inline]
    pub fn rfc3339(&self) -> String {
        format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}{}",
            self.year(),
            self.month(),
            self.day(),
            self.hour(),
            self.minute(),
            self.second(),
            self.zone_offset()
        )
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DatetimeNowResult {
    pub datetime: String,
    pub timestamp: String,
}

impl DatetimeNowResult {
    #[inline]
    pub fn new(datetime: String, timestamp: String) -> Self {
        Self {
            datetime,
            timestamp,
        }
    }
}
