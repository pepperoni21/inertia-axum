use std::sync::Arc;

use axum::{
    extract::{Request, State},
    response::Response,
    routing::get,
    Router,
};
use inertia_axum::{render_with_props, InertiaConfig};
use serde::Serialize;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let inertia_config = InertiaConfig::new("index.html".into());

    let serve_dir = ServeDir::new("client/dist/assets");
    let app = Router::new()
        .route("/", get(root))
        .nest_service("/public", serve_dir)
        .with_state(Arc::new(inertia_config));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root(State(inertia_config): State<Arc<InertiaConfig>>, request: Request) -> Response {
    render_with_props(
        &inertia_config,
        &request,
        "index".into(),
        RootData {
            user: "pepperoni21".into(),
        },
    )
}

#[derive(Serialize)]
struct RootData {
    user: String,
}
