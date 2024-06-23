use axum::{body::Bytes, extract::State, http::StatusCode, routing::post, Router};
use miette::{IntoDiagnostic, Result};
use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(Debug, Clone)]
struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:test@localhost/mainframe")
        .await
        .into_diagnostic()?;

    let shared_state = AppState { db };

    // build our application with a single route
    let app = Router::new()
        .route("/", post(handler))
        .with_state(shared_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn handler(State(state): State<AppState>, body: Bytes) -> Result<String, StatusCode> {

    println!("Received body: {}", String::from_utf8_lossy(&body));

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&state.db).await
        .map_err(|e| {
            println!("Failed fetching data from db: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
        )?;

    Ok(format!("Fetched {}", row.0))
}
