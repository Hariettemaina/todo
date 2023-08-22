use async_graphql::MergedObject;
pub mod add_category;

pub use add_category::AddCategoryMutation;

#[derive(MergedObject, Default)]
pub struct Mutation(pub AddCategoryMutation);
