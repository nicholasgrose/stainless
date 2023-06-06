use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::{Extension, Router};
use axum::routing::get;
use axum_server::tls_rustls::RustlsConfig;

use crate::database::DatabaseContext;
use crate::web::routes::graphql;
use crate::web::schema::QueryRoot;

pub mod routes;
pub mod schema;

pub async fn start_server(address: &str) -> std::io::Result<()> {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(DatabaseContext::new("iron_db.sqlite3").unwrap())
        .finish();

    let app = Router::new()
        .route("/graphql", get(graphql).post(graphql))
        .layer(Extension(schema));
    let tls_config = RustlsConfig::from_pem_file("cert.pem", "key.pem").await?;

    axum_server::bind_rustls(address.parse().unwrap(), tls_config)
        .serve(app.into_make_service())
        .await
}
