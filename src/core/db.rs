use deadpool_diesel::{Manager, Pool};
use diesel::prelude::*;
use log::info;

pub fn establish_connection(db_url: String) -> Pool<Manager<PgConnection>> {
    info!("Connecting to postgres.");

    let manager: Manager<PgConnection> = Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    let pool: Pool<Manager<PgConnection>> = Pool::builder(manager)
        .build()
        .unwrap();

    info!("Connected.");

    pool  // Return the Connection Pool
}

pub fn dop_connection() {
    /*
        For now dropping the connection is my concern. Because Rust's memory management can handle
        that.
    */
}
