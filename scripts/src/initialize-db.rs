use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use dotenv::dotenv;
use std::env;

async fn do_migration(pool: &Pool<Postgres>) {
    sqlx::migrate!("./migrations/")
        .run(pool).await.unwrap()
}

async fn initialize_db(pool: Pool<Postgres>) {
    do_migration(&pool).await;
}

#[tokio::main]
async fn main() -> Result<(), String> {
    dotenv().ok();

    let databse_url = env::var("DATABASE_URL").map_err(|_| "Missing env variable DATABASE_URL")?;

    println!("Initizlizing db {}", databse_url);

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(databse_url.as_str()).await.map_err(|err| err.to_string())?;

    initialize_db(pool).await;

    Ok(())
}
