use diesel::{PgConnection, QueryResult};
use crate::todo::models::Label;
use crate::todo::services::TodoService;

pub async fn get_labels(conn: PgConnection) {
    let mut todo_srv = TodoService { conn };
    let labels: QueryResult<Vec<Label>> = todo_srv.list_labels().await;
    println!("{:?}", labels);
}

