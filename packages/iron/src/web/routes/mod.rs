use actix_web::{
    get, route,
    web::{Data, Payload},
    HttpRequest, HttpResponse,
};
use juniper_actix::{graphiql_handler, graphql_handler, playground_handler};

use crate::{database::Database, web::schema::Schema};

#[route("/graphql", method = "GET", method = "POST")]
pub async fn graphql(
    req: HttpRequest,
    payload: Payload,
    schema: Data<Schema>,
) -> actix_web::Result<HttpResponse> {
    let context = Database::new();
    graphql_handler(&schema, &context, req, payload).await
}

#[get("/graphiql")]
pub async fn graphiql() -> actix_web::Result<HttpResponse> {
    graphiql_handler("/graphql", None).await
}

#[get("/playground")]
pub async fn playground() -> actix_web::Result<HttpResponse> {
    playground_handler("/graphql", None).await
}
