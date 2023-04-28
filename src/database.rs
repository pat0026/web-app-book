use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::config::Config;

pub fn establish_connection() -> PgConnection {
    let config = Config::new();
    let database_url_value = config.map.get("DB_URL")
        .expect("DATABASE_URL must be set");
    let database_url: String = serde_yaml::from_value(database_url_value.to_owned())
        .unwrap();
    PgConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}