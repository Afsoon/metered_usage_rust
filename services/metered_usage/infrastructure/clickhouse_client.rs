use std::fmt;

use clickhouse::Client;

pub struct ClientWrapper(Client);

impl fmt::Debug for ClientWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ClientClikhouseWrapper").finish()
    }
}

impl std::ops::Deref for ClientWrapper {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct ClickhouseClient {
    connection: ClientWrapper,
}

impl ClickhouseClient {
    pub fn new() -> Self {
        let clickhouse_client = Client::default()
            .with_url("http://localhost:8123/")
            .with_user("default")
            .with_password("admin")
            .with_database("analytics");

        return ClickhouseClient {
            connection: ClientWrapper(clickhouse_client),
        };
    }

    pub fn clone(&self) -> ClientWrapper {
        return ClientWrapper(self.connection.0.clone());
    }
}
