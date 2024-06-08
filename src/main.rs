use axum::Router;
use dotenv::dotenv;
use konnektoren_api::routes;
use routes::openapi::ApiDoc;
use std::net::SocketAddr;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv().ok();
    let app = Router::new()
        .nest("/api/v1", routes::v1::create_router())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    log::info!("Server running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
