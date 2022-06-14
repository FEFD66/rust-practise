use std::env;

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

#[test]
fn getx() {
    dotenv().ok();
    let pgurl=env::var("PGURL").unwrap();
    println!("Hello, Sqlx!");
    // tokio runtime
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all().build().unwrap();
    runtime.block_on( async {
        connect(&pgurl)
            .await.unwrap();
    });
}
async fn connect(pgurl: &str)->Result<(), sqlx::Error>{
    let pool =PgPoolOptions::new()
        .max_connections(5)
        .connect(pgurl).await?;
    sqlx::query(r#"CREATE TABLE IF NOT EXISTS person (
            id bigserial,
            name varchar NOT NULL
            );"#
        ).execute(&pool)
        .await?;
    Ok(())
}
