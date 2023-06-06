use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::Extension;

use crate::web::schema::IronSchema;

// #[debug_handler(state = Schema)]
pub async fn graphql(
    Extension(schema): Extension<IronSchema>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(request.into_inner()).await.into()
}
