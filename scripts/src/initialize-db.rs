use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use dotenv::dotenv;
use std::env;

use abis::contracts::ABIS;

async fn do_migration(pool: &Pool<Postgres>) {
    sqlx::migrate!("./migrations/").run(pool).await.unwrap()
}

async fn initialize_db(pool: Pool<Postgres>) {
    do_migration(&pool).await;

    let mut futures = Vec::new();
    let mut mapping_inserts: Vec<sqlx::query::Query<'_, Postgres, _>> = Vec::new();

    for interface_name in ABIS.keys() {
        let abi = ABIS.get(interface_name).unwrap();
        futures.push(sqlx::query(
                "INSERT INTO interface (name) VALUES ($1)")
                .bind(interface_name)
                .execute(&pool));

        for event in abi.events() {
            let signature = event.signature();
            let selector = event.selector().as_slice().to_owned(); // Create a longer-lived value
            futures.push(sqlx::query(
                "INSERT INTO event_type (signature, selector) VALUES ($1, $2)"
                )
                .bind(signature.clone())
                .bind(selector)
                .execute(&pool)
            );


            mapping_inserts.push(sqlx::query(
                "INSERT INTO mapping_event_interface (name, signature) VALUES ($1, $2)"
                )
                .bind(interface_name)
                .bind(signature)
            );
        }
    }

    for future in futures.into_iter() {
        let _ = future.await;
    }

    futures = Vec::new();

    for insert in mapping_inserts {
        futures.push(insert.execute(&pool));
    }

    for future in futures.into_iter() {
        let _ = future.await;
    }
}

#[tokio::main]
async fn main() -> Result<(), String> {
    dotenv().ok();

    let databse_url = env::var("DATABASE_URL").map_err(|_| "Missing env variable DATABASE_URL")?;

    println!("Initizlizing db {}", databse_url);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(databse_url.as_str())
        .await
        .map_err(|err| err.to_string())?;

    initialize_db(pool).await;

    Ok(())
}
