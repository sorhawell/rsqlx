use extendr_api::prelude::*;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::result::Result;
use tokio;
use tokio::runtime::Runtime;

#[extendr]
fn hello_world() -> &'static str {
    "Hello world! 123"
}

pub struct RSQLXConnection {
    pool: Pool<Postgres>,
    rt: Runtime,
}

async fn make_connection(conn_str: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(conn_str)
        .await?;
    Ok(pool)
}

#[extendr]
impl RSQLXConnection {
    fn new(conn_str: &str) -> Result<Self, String> {
        // Create a new Tokio runtime
        let rt = Runtime::new().map_err(|err| format!("Failed to create runtime: {:?}", err))?;

        // Run the async code inside the runtime
        let pool_result = rt.block_on(make_connection(conn_str));

        pool_result
            .map_err(|err| format!("failed to make connection because {:?}", err))
            .map(|pool| RSQLXConnection { pool, rt })
    }

    fn fetch_qeury(&self, query: &str) -> Result<Vec<String>, String> {
        let pool = self.pool.clone();
        let rows = self
            .rt
            .block_on(sqlx::query(&query).fetch_all(&pool))
            .map_err(|e| format!("{:?}", e))?;
        Ok(rows.into_iter().map(|row| format!("{:?}", row)).collect())
    }

    fn fetch_qeury_background(&self, query: String) -> RSQLXFuture {
        // reserve connection
        let pool = self.pool.clone();

        let handle = std::thread::spawn(|| -> Result<Vec<String>, String> {
            let query = query;
            let pool = pool;
            let rt =
                Runtime::new().map_err(|err| format!("Failed to create runtime: {:?}", err))?;
            let rows = rt
                .block_on(sqlx::query(&query).fetch_all(&pool))
                .map_err(|e| format!("{:?}", e))?;
            Ok(rows.into_iter().map(|row| format!("{:?}", row)).collect())
        });

        RSQLXFuture {
            handle: Some(handle),
        }
    }

    fn drop(&mut self) {
        rprintln!("Closing all sqlx connection");
        self.rt.block_on(self.pool.close());
    }
}

pub struct RSQLXFuture {
    handle: Option<std::thread::JoinHandle<Result<Vec<String>, String>>>,
}

#[extendr]
impl RSQLXFuture {
    fn join(&mut self) -> Result<Vec<String>, String> {
        self.handle
            .take()
            .ok_or_else(|| "handle already exhausted".to_string())?
            .join()
            .map_err(|err| format!("{:?}", err))?
    }

    fn is_done(&self) -> bool {
        self.handle.is_some()
    }
}

async fn read_table_internal(conn_str: &str, table: &str) -> Result<Vec<String>, sqlx::Error> {
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

    pool.close().await;

    Ok(result)
}

// Wrap for R
#[extendr]
fn r_read_table(conn_str: &str, table: &str) -> Result<Vec<String>, String> {
    let rt = tokio::runtime::Runtime::new().map_err(|err| err.to_string())?;
    rt.block_on(read_table_internal(conn_str, table))
        .map_err(|err| err.to_string())
}

extendr_module! {
    mod rsqlx;
    fn r_read_table;
    fn hello_world;
    impl RSQLXConnection;
    impl RSQLXFuture;
}
