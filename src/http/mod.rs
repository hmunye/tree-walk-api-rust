use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post, IntoMakeService};
use axum::serve::Serve;
use axum::{extract, Router};
use serde::Deserialize;

type Server = Serve<IntoMakeService<Router>, Router>;

#[allow(dead_code)]
#[derive(Deserialize)]
struct TreePayload {
    latitude: f32,
    longitude: f32,
    address: String,
    dbh: f32,
    species: String,
    description: String,
}

async fn create_tree(extract::Json(payload): extract::Json<TreePayload>) -> impl IntoResponse {
    let _ = payload;
    (StatusCode::OK, "Successfully Created Tree").into_response()
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "Server Running").into_response()
}

pub fn serve(listener: tokio::net::TcpListener) -> Result<Server, std::io::Error> {
    let routes = Router::new()
        .route("/v1/healthcheck", get(health_check))
        .route("/v1/trees", post(create_tree));

    let server = axum::serve(listener, routes.into_make_service());

    Ok(server)
}
