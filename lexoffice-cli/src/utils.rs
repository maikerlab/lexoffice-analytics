use chrono::{DateTime, Utc};

pub fn parse_date_string(date_str: String) -> Option<DateTime<Utc>> {
    DateTime::parse_from_str(
        format!("{} 00:00:00.000 +0000", date_str).as_str(), "%Y-%m-%d %H:%M:%S%.3f %z"
    ).map(|dt| dt.to_utc()).ok()
}