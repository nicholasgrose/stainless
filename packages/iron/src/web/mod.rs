use crate::config::IronConfig;
use anyhow::Context;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::routing::get;
use axum::{Extension, Router};
use axum_server::tls_rustls::RustlsConfig;
use sea_orm::Database;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;

use crate::web::routes::{graphiql, graphql, playground};
use crate::web::schema::query::IronQueryRoot;

pub mod routes;
pub mod schema;

pub async fn start_server(config: IronConfig) -> anyhow::Result<()> {
    let schema = Schema::build(IronQueryRoot, EmptyMutation, EmptySubscription)
        .data(Database::connect(config.database_uri).await?)
        .finish();
    let service = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new());

    let app = Router::new()
        .route("/graphql", get(graphql).post(graphql))
        .route("/graphiql", get(graphiql))
        .route("/playground", get(playground))
        .layer(service)
        .layer(Extension(schema));
    let tls_config =
        RustlsConfig::from_pem_file(config.tls.certificate_file_path, config.tls.key_file_path)
            .await?;

    axum_server::bind_rustls(config.address, tls_config)
        .serve(app.into_make_service())
        .await
        .with_context(|| "Server experienced an error during execution")
}
