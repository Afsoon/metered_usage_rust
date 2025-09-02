use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::repository::models::{Row, UsageType};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeteredUsageEvent {
    #[serde(with = "uuid::serde::simple")]
    customer_id: uuid::Uuid,
    usage_type: MeteredUsageEventType,
    created_at: DateTime<Utc>,
}

impl MeteredUsageEvent {
    pub fn random() -> Self {
        MeteredUsageEvent {
            customer_id: Uuid::new_v4(),
            usage_type: MeteredUsageEventType::ApiCall,
            created_at: chrono::offset::Utc::now(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MeteredUsageEventType {
    ApiCall = 1,
    Storage = 2,
    ExecutionTime = 3,
}

impl From<MeteredUsageEventType> for UsageType {
    fn from(event_type: MeteredUsageEventType) -> UsageType {
        match event_type {
            MeteredUsageEventType::ApiCall => UsageType::ApiCall,
            MeteredUsageEventType::Storage => UsageType::Storage,
            MeteredUsageEventType::ExecutionTime => UsageType::ExecutionTime,
        }
    }
}

impl Into<Row> for MeteredUsageEvent {
    fn into(self) -> Row {
        Row {
            created_at: self.created_at,
            customer_id: self.customer_id,
            usage_type: self.usage_type.into(),
        }
    }
}
