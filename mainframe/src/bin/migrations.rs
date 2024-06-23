use miette::{Context, IntoDiagnostic, Result};
use sqlx::{migrate::Migrator, postgres::PgPoolOptions};

static MIGRATOR: Migrator = sqlx::migrate!(); // defaults to "./migrations"

#[tokio::main]
async fn main() -> Result<()> {
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:test@localhost/mainframe")
        .await
        .into_diagnostic()
        .wrap_err("Failed connecting to the database")?;

    MIGRATOR
        .run(&db)
        .await
        .into_diagnostic()
        .wrap_err("Failed running migrations")?;

    println!("Migrations applied successfully");
    Ok(())
}
