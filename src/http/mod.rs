use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, IntoMakeService};
use axum::serve::Serve;
use axum::Router;

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "Server Running").into_response()
}

pub fn serve(listener: tokio::net::TcpListener) -> Serve<IntoMakeService<Router>, Router> {
    let routes = Router::new().route("/v1/healthcheck", get(health_check));

    axum::serve(listener, routes.into_make_service())
}
