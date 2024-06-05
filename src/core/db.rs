use diesel::prelude::*;
use log::info;

pub fn establish_connection(db_url: String) -> PgConnection {
    info!("Connecting to postgres.");

    let conn: PgConnection = PgConnection::establish(&db_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_url));

    info!("Connected.");

    conn  // Return the PgConnection
}

pub fn dop_connection() {
    /*
        For now dropping the connection is my concern. Because Rust's memory management can handle
        that.
    */
}
