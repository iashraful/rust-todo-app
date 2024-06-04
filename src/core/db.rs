use std::env;

use diesel::prelude::*;
use log::info;

pub fn establish_connection() -> PgConnection {
    info!("Connecting to postgres.");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn: PgConnection = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    info!("Connected.");

    conn  // Return the PgConnection
}

pub fn dop_connection() {
    /*
        For now dropping the connection is my concern. Because Rust's memory management can handle
        that.
    */
}
