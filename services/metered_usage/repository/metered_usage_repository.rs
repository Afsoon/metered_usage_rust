use std::io::Error;

use clickhouse::Client;
use tracing::{Level, event, instrument};

use crate::repository::models::Row;

#[derive(Debug)]
pub struct MeteredUsageRepository {}

impl MeteredUsageRepository {
    pub fn new() -> Self {
        return MeteredUsageRepository {};
    }

    #[instrument(skip(self, db_client))]
    pub async fn insert<'c>(&self, event: Row, db_client: &'c Client) -> Result<(), Error> {
        event!(Level::INFO, "Starting transaction");
        let mut insert = db_client.insert("metered_usage")?;

        insert.write(&event).await?;
        event!(Level::INFO, "Before flusing");

        insert.end().await?;
        event!(Level::INFO, "Ended transaction");

        return Ok(());
    }
}
