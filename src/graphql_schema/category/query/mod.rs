use async_graphql::MergedObject;

pub mod get_category;

use get_category::CategoryQuery;

#[derive(MergedObject, Default)]
pub struct CategorysQuery(pub CategoryQuery);
