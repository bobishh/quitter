use axum::{
    routing::get,
    extract::{State, Json},
    body::Bytes,
    http::StatusCode,
    Router,
};
use prost::Message;
use shared::tracker::{Theme as ProtoTheme, Habit as ProtoHabit};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, FromRow};
use std::net::SocketAddr;
use tower_http::services::{ServeDir, ServeFile};

#[derive(Serialize, Deserialize, Clone, FromRow)]
struct Theme {
    id: uuid::Uuid,
    name: String,
    css: String,
    icon_limit: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, FromRow)]
struct Habit {
    id: uuid::Uuid,
    slug: String,
    name: String,
    icon: String,
    unit_name: String,
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

    let frontend_dist = std::env::var("FRONTEND_DIST")
        .unwrap_or_else(|_| "../frontend/dist".to_string());

    let app = Router::new()
        .route("/api/habits", get(get_habits).post(create_habit))
        .route("/api/themes", get(get_themes).post(create_theme))
        .fallback_service(
            ServeDir::new(&frontend_dist)
                .not_found_service(ServeFile::new(format!("{}/index.html", frontend_dist))),
        )
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

async fn get_themes(State(state): State<AppState>) -> Json<Vec<Theme>> {
    let themes = sqlx::query_as::<_, Theme>("SELECT id, name, css, icon_limit FROM themes")
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();
    Json(themes)
}

async fn create_theme(State(state): State<AppState>, body: Bytes) -> Result<Json<Theme>, StatusCode> {
    let proto_theme = ProtoTheme::decode(body).map_err(|_| StatusCode::BAD_REQUEST)?;
    
    if proto_theme.name.trim().is_empty() || proto_theme.css.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    if let Some(limit) = proto_theme.icon_limit {
        if limit < 0 {
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    let id = uuid::Uuid::parse_str(&proto_theme.id).map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let theme = Theme {
        id,
        name: proto_theme.name,
        css: proto_theme.css,
        icon_limit: proto_theme.icon_limit,
    };

    sqlx::query("INSERT INTO themes (id, name, css, icon_limit) VALUES ($1, $2, $3, $4)")
        .bind(theme.id)
        .bind(&theme.name)
        .bind(&theme.css)
        .bind(theme.icon_limit)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
    Ok(Json(theme))
}

async fn get_habits(State(state): State<AppState>) -> Json<Vec<Habit>> {
    println!("Fetching habits from DB...");
    let result = sqlx::query_as::<_, Habit>(
        "SELECT id, slug, name, icon, unit_name FROM habits"
    )
    .fetch_all(&state.db)
    .await;

    match result {
        Ok(habits) => {
            println!("Found {} habits", habits.len());
            Json(habits)
        }
        Err(e) => {
            println!("Error fetching habits: {:?}", e);
            Json(vec![])
        }
    }
}

async fn create_habit(State(state): State<AppState>, body: Bytes) -> Result<Json<Habit>, StatusCode> {
    let proto = ProtoHabit::decode(body).map_err(|_| StatusCode::BAD_REQUEST)?;
    
    if proto.name.trim().is_empty() 
        || proto.slug.trim().is_empty() 
        || proto.icon.trim().is_empty() 
        || proto.unit_name.trim().is_empty() 
    {
        return Err(StatusCode::BAD_REQUEST);
    }

    let id = uuid::Uuid::parse_str(&proto.id).map_err(|_| StatusCode::BAD_REQUEST)?;

    let habit = Habit {
        id,
        slug: proto.slug,
        name: proto.name,
        icon: proto.icon,
        unit_name: proto.unit_name,
    };

    sqlx::query(
        "INSERT INTO habits (id, slug, name, icon, unit_name) VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(habit.id)
    .bind(&habit.slug)
    .bind(&habit.name)
    .bind(&habit.icon)
    .bind(&habit.unit_name)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(habit))
}