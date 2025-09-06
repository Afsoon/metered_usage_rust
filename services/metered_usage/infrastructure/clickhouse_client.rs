use std::env;
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
        let clickhouse_config = ClickhouseConfig::new();

        let clickhouse_client = Client::default()
            .with_url(clickhouse_config.url)
            .with_user(clickhouse_config.user)
            .with_password(clickhouse_config.password)
            .with_database(clickhouse_config.database);

        return ClickhouseClient {
            connection: ClientWrapper(clickhouse_client),
        };
    }

    pub fn clone(&self) -> ClientWrapper {
        return ClientWrapper(self.connection.0.clone());
    }
}

struct ClickhouseConfig {
    url: String,
    user: String,
    password: String,
    database: String,
}

impl ClickhouseConfig {
    pub fn new() -> Self {
        let url = match env::var("CLICKHOUSE_URL") {
            Ok(url) => url,
            Err(_) => "http://localhost:8123".into(),
        };

        let user = match env::var("CLICKHOUSE_USER") {
            Ok(user) => user,
            Err(_) => "default".into(),
        };

        let password = match env::var("CLICKHOUSE_PASSWORD") {
            Ok(password) => password,
            Err(_) => "admin".into(),
        };

        let database = match env::var("CLICKHOUSE_DATABASE") {
            Ok(database) => database,
            Err(_) => "analytics".into(),
        };

        return ClickhouseConfig {
            url,
            user,
            password,
            database,
        };
    }
}
