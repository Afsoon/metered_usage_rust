use std::io::Error;

use clickhouse::Client;

use crate::{
    repository::metered_usage_repository::MeteredUsageRepository,
    services::entities::MeteredUsageEvent,
};

pub struct MeteredUsageService {
    pub repository: MeteredUsageRepository,
    pub db_client: Client,
}

impl MeteredUsageService {
    pub fn new(db_client: Client) -> MeteredUsageService {
        return MeteredUsageService {
            repository: MeteredUsageRepository::new(),
            db_client,
        };
    }

    pub async fn insert_metered_event(
        &self,
        event: MeteredUsageEvent,
        db_client: &Client,
    ) -> Result<(), Error> {
        println!("Before inserting");
        self.repository.insert(event.into(), &db_client).await?;
        println!("After inserting");

        return Ok(());
    }
}
