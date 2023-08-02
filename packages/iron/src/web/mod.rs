use anyhow::Context;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::routing::get;
use axum::{Extension, Router};
use axum_server::tls_rustls::RustlsConfig;
use sea_orm::Database;

use crate::web::routes::{graphiql, graphql, playground};
use crate::web::schema::query::QueryRoot;

pub mod routes;
pub mod schema;

pub async fn start_server(address: &str) -> anyhow::Result<()> {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(Database::connect("sqlite://iron_db.sqlite3").await?)
        .finish();

    let app = Router::new()
        .route("/graphql", get(graphql).post(graphql))
        .route("/graphiql", get(graphiql))
        .route("/playground", get(playground))
        .layer(Extension(schema));
    let tls_config = RustlsConfig::from_pem_file("cert.pem", "key.pem").await?;

    axum_server::bind_rustls(address.parse()?, tls_config)
        .serve(app.into_make_service())
        .await
        .with_context(|| format!("Server running on {} experienced an error", address))
}
