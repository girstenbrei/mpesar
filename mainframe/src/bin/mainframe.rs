use axum::{body::Bytes, extract::State, http::StatusCode, routing::post, Router};
use miette::{IntoDiagnostic, Result};
use serde::Deserialize;
use serde_json::from_slice;
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool};
//use rust_decimal::Decimal

#[derive(Debug, Clone)]
struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:test@localhost:5432/mainframe")
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

#[derive(Deserialize)]
struct RequestBody {
    account_id: i32,
    ussd_code: String,
}

#[derive(FromRow)]
struct Account {
    account_id: i32,
    //balance: Decimal
}


async fn handler(State(state): State<AppState>, body: Bytes) -> Result<String, StatusCode> {
    println!("Received body: {}", String::from_utf8_lossy(&body));

    let request_body: RequestBody = from_slice(&body).map_err(|e| StatusCode::BAD_REQUEST)?;

    let account_id: i32 = request_body.account_id;
    let ussd_code = request_body.ussd_code;

    println!("Received Account ID {}", account_id);
    println!("Received USSD Code {}", ussd_code);

    let account: Account = sqlx::query_as("SELECT * FROM mainframe.accounts WHERE account_id = $1")
        .bind(account_id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| {
            println!("Failed fetching data from db: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(format!("Received Account"))
}
