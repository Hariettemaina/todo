use async_graphql::MergedObject;

pub mod get_todos;

use get_todos::TodoQuery;

#[derive(MergedObject, Default)]
pub struct TodosQuery(pub TodoQuery);
