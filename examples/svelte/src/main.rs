use std::sync::Arc;

use axum::{
    extract::Request,
    middleware::{from_fn, Next},
    response::{IntoResponse, Response},
    routing::get,
    Extension, Router,
};
use inertia_axum::{
    add_shared_state, set_assets_version, AssetsVersion, InertiaConfig, InertiaRenderer,
};
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
        .route("/counter", get(counter))
        .nest_service("/public", serve_dir)
        .route_layer(from_fn(shared_state_middleware))
        .route_layer(from_fn(assets_versioning_middleware))
        .layer(Extension(Arc::new(inertia_config)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn shared_state_middleware(mut req: Request, next: Next) -> Response {
    match req.uri().path() {
        "/" | "counter" => add_shared_state(
            &mut req,
            serde_json::json!({
                "user": "John Doe"
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
        _ => {}
    };
    next.run(req).await
}

async fn assets_versioning_middleware(mut req: Request, next: Next) -> Response {
    match req.uri().path() {
        "/counter" => set_assets_version(&mut req, AssetsVersion::String("counter".into())),
        "/" => set_assets_version(&mut req, AssetsVersion::String("root".into())),
        _ => {}
    };
    next.run(req).await
}

async fn root(
    Extension(inertia_config): Extension<Arc<InertiaConfig>>,
    request: Request,
) -> Response {
    InertiaRenderer::render("index", &request, &inertia_config)
        .with_props(RootData {
            message: "Hey".into(),
        })
        .into_response()
}

#[derive(Serialize)]
struct RootData {
    message: String,
}

async fn counter(
    Extension(inertia_config): Extension<Arc<InertiaConfig>>,
    request: Request,
) -> Response {
    InertiaRenderer::render("counter", &request, &inertia_config).into_response()
}
