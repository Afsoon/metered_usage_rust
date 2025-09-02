use clickhouse::Client;

pub struct ClickhouseClient {
    connection: Client,
}

impl ClickhouseClient {
    pub fn new() -> Self {
        let clickhouse_client = Client::default()
            .with_url("http://localhost:8123/")
            .with_user("default")
            .with_password("admin")
            .with_database("analytics");

        return ClickhouseClient {
            connection: clickhouse_client,
        };
    }

    pub fn clone(&self) -> Client {
        return self.connection.clone();
    }
}
