use actix_web::{Error, get, HttpRequest, HttpResponse, route, web::{Data, Payload}};
use juniper_actix::{graphiql_handler, graphql_handler, playground_handler};

use database::Database;
use schema::Schema;

use crate::{database, schema};

#[route("/graphql", method = "GET", method = "POST")]
pub async fn graphql(
    req: HttpRequest,
    payload: Payload,
    schema: Data<Schema>,
) -> Result<HttpResponse, Error> {
    let context = Database::new();
    graphql_handler(&schema, &context, req, payload).await
}

#[get("/graphiql")]
pub async fn graphiql() -> Result<HttpResponse, Error> {
    graphiql_handler("/graphql", None).await
}

#[get("/playground")]
pub async fn playground() -> Result<HttpResponse, Error> {
    playground_handler("/graphql", None).await
}
