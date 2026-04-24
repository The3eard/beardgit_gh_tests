use std::str::FromStr;

use anyhow::{anyhow, Error};
use chrono::{Datelike, Months, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Recurrence {
    Daily,
    Weekly,
    Monthly,
}

impl Recurrence {
    pub fn next_after(self, anchor: NaiveDate) -> NaiveDate {
        match self {
            Recurrence::Daily => anchor.succ_opt().unwrap_or(anchor),
            Recurrence::Weekly => anchor + chrono::Duration::weeks(1),
            Recurrence::Monthly => anchor
                .checked_add_months(Months::new(1))
                .unwrap_or_else(|| {
                    NaiveDate::from_ymd_opt(anchor.year(), anchor.month(), 28).unwrap_or(anchor)
                }),
        }
    }
}

impl FromStr for Recurrence {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "daily" | "day" | "d" => Ok(Self::Daily),
            "weekly" | "week" | "w" => Ok(Self::Weekly),
            "monthly" | "month" | "m" => Ok(Self::Monthly),
            other => Err(anyhow!(
                "unknown recurrence '{other}'; expected daily|weekly|monthly"
            )),
        }
    }
}
