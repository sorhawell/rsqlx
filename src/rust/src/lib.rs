use extendr_api::prelude::*;
use sqlx::{postgres::PgPoolOptions, Postgres};
use tokio;

use std::result::Result;

#[extendr]
fn hello_world() -> &'static str {
    "Hello world! 123"
}

async fn read_table(conn_str: &str, table: &str) -> Result<Vec<String>, sqlx::Error> {
    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(conn_str)
        .await?;

    // Perform query to read the table
    let query = format!("SELECT * FROM {}", table);
    let rows = sqlx::query(&query).fetch_all(&pool).await?;

    // Format the result as a vector of strings
    let result = rows
        .into_iter()
        .map(|row| format!("{:?}", row)) // Convert row data to string
        .collect();

    Ok(result)
}

// Wrap for R
#[extendr]
fn r_read_table(conn_str: &str, table: &str) -> Robj {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(read_table(conn_str, table)); // Now it properly awaits the async function

    match result {
        Ok(data) => Robj::from(data),
        Err(err) => Robj::from(format!("Error: {:?}", err)),
    }
}

extendr_module! {
    mod rsqlx;
    fn r_read_table;
    fn hello_world;
}
