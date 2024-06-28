mod member;

use std::{fs, path::Path};

use axum::{response::IntoResponse, routing::get};
use member::{create_member, list_members};

const DB_PATH: &str = "./sqlite/data/database.db";

#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt::init();

    init_db_file().expect("failed to initialize database file");

    let conn = db::connect(format!("sqlite:{}", DB_PATH).as_str())
        .await
        .expect("failed to connect to database");

    db::refresh(&conn)
        .await
        .expect("failed to refresh database");

    let state = AppState { conn };

    let app = axum::Router::new()
        .route("/", get(root))
        .route("/member", get(list_members).post(create_member))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind to address");
    axum::serve(listener, app).await.expect("server failed");
}

fn init_db_file() -> Result<(), std::io::Error> {
    let db_dir = Path::new(DB_PATH)
        .parent()
        .expect("failed to get parent directory");

    if !db_dir.exists() {
        fs::create_dir_all(db_dir)?;
    }

    if !Path::new(DB_PATH).exists() {
        println!("Creating database file at {}", DB_PATH);
        fs::File::create(DB_PATH)?;
    }

    Ok(())
}

#[derive(Clone)]
struct AppState {
    conn: db::DbConn,
}

async fn root() -> impl IntoResponse {
    "Hello, World!"
}
