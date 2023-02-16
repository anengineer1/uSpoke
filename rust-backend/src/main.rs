use sqlx::postgres::PgPoolOptions;
// use sqlx::mysql::MySqlPoolOptions;
// etc.

// #[async_std::main]
#[tokio::main]
// or #[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool
    //  for MySQL, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@127.0.0.1/postgres")
        .await?;

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, 150);
    println!("It works! Debug Print of pool: {:#?}", pool);

    Ok(())
}
