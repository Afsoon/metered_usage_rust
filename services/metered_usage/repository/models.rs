// GENERATED CODE (ch2rs v0.1.8)
#![cfg_attr(rustfmt, rustfmt::skip)]
#![allow(warnings)]
#![allow(clippy::all)]

// Generated with the following options:
/*
ch2rs metered_usage -d analytics \
        --derive Clone \
        --derive PartialEq \
        -T 'DateTime64(3)=DateTime<Utc>'
*/

use serde::{Deserialize, Serialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use chrono::{DateTime, Utc};


#[derive(Debug, clickhouse::Row)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Row {
    #[serde(with = "clickhouse::serde::uuid")]
    pub customer_id: uuid::Uuid,
    pub usage_type: UsageType,
    #[serde(with = "clickhouse::serde::chrono::datetime64::millis")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[repr(i16)]
#[derive(Serialize_repr)]
#[derive(Deserialize_repr)]
pub enum UsageType {
    ApiCall = 1,
    Storage = 2,
    ExecutionTime = 3,
}
