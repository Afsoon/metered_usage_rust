use chrono::{DateTime, Utc};
use metered_usage::services::entities::{MeteredUsageEvent, MeteredUsageEventType};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct MeteredUsagePayload {
    customerId: Uuid,
    usageType: MeteredUsageEventType,
    createdAt: DateTime<Utc>,
}

impl Into<MeteredUsageEvent> for MeteredUsagePayload {
    fn into(self) -> MeteredUsageEvent {
        MeteredUsageEvent {
            customer_id: self.customerId,
            usage_type: self.usageType,
            created_at: self.createdAt,
        }
    }
}
