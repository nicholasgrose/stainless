use async_graphql::{EmptyMutation, EmptySubscription};

use crate::web::schema::query::QueryRoot;

pub mod game;
pub mod query;

pub type IronSchema = async_graphql::Schema<QueryRoot, EmptyMutation, EmptySubscription>;
