use std::{fmt::Display, str::FromStr};

use chrono::{Datelike, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Date {
    year: i32,
    month: u32,
    day: u32,
}

impl TryFrom<NaiveDateTime> for Date {
    type Error = String;
    fn try_from(value: NaiveDateTime) -> Result<Self, Self::Error> {
        Ok(Self {
            year: value.year(),
            month: value.month(),
            day: value.day(),
        })
    }
}

impl TryFrom<String> for Date {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let chrono_date = NaiveDateTime::from_str(&value)
            .map_err(|_| "could not parse a string to Date".to_string())?;
        Ok(Self {
            year: chrono_date.year(),
            month: chrono_date.month(),
            day: chrono_date.day(),
        })
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}-{}", { self.day }, { self.month }, { self.year })
    }
}
