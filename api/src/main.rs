use axum::{
    routing::{get, post},
    Json, Router,
    extract::State,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, FromRow};
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use shared::tracker::TrackerState;

#[derive(Serialize, Deserialize, Clone, FromRow)]
struct Habit {
    id: uuid::Uuid,
    name: String,
    icon: String,
    theme_color: String,
    unit_name: String,
    frequency_hours: f64,
}

#[derive(Clone)]
struct AppState {
    db: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost/addict".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let state = AppState { db: pool };

    let app = Router::new()
        .route("/api/habits", get(get_habits).post(create_habit))
        .nest_service("/", ServeDir::new("../frontend/dist"))
        .with_state(state);

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Server running on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_habits(State(state): State<AppState>) -> Json<Vec<Habit>> {
    let habits = sqlx::query_as::<_, Habit>(
        "SELECT id, name, icon, theme_color, unit_name, frequency_hours FROM habits"
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    Json(habits)
}

async fn create_habit(State(state): State<AppState>, Json(payload): Json<Habit>) -> Json<Habit> {
    sqlx::query(
        "INSERT INTO habits (id, name, icon, theme_color, unit_name, frequency_hours) VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(payload.id)
    .bind(&payload.name)
    .bind(&payload.icon)
    .bind(&payload.theme_color)
    .bind(&payload.unit_name)
    .bind(payload.frequency_hours)
    .execute(&state.db)
    .await
    .unwrap();

    Json(payload)
}