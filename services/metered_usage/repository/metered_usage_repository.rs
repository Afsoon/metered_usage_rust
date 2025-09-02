use std::io::Error;

use clickhouse::Client;

use crate::repository::models::Row;

pub struct MeteredUsageRepository {}

impl MeteredUsageRepository {
    pub fn new() -> Self {
        return MeteredUsageRepository {};
    }

    pub async fn insert<'c>(&self, event: Row, db_client: &'c Client) -> Result<(), Error> {
        println!("Starting transaction");
        let mut insert = db_client.insert("metered_usage")?;

        insert.write(&event).await?;
        println!("Beflor flusing");

        insert.end().await?;
        println!("Ending transaction");

        return Ok(());
    }
}
