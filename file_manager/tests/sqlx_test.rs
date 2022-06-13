use sqlx::postgres::PgPoolOptions;


#[test]
fn getx() {
    println!("Hello, Sqlx!");
    // tokio runtime
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all().build().unwrap();
    runtime.block_on( async {
        connect()
            .await.unwrap();
    });
}
async fn connect()->Result<(), sqlx::Error>{
    let pool =PgPoolOptions::new()
        .max_connections(5)
        .connect(r#"postgresql://file:filefile@sh-tdcpg-ep-24rz9f8y.sql.tencentcdb.com:28867/file"#).await?;
    sqlx::query(r#"CREATE TABLE IF NOT EXISTS person (
            id bigserial,
            name varchar NOT NULL
            );"#
        ).execute(&pool)
        .await?;
    Ok(())
}


